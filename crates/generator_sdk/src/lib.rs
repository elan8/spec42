//! Rust guest SDK for Spec42's core WebAssembly generator ABI.
//!
//! The full wire contract is specified in `docs/generation/ABI.md`. This crate is one
//! implementation of it, not the definition — a guest in any language that can produce the
//! documented imports and exports is equally valid.

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use spec42_generator_protocol as protocol;
pub use spec42_generator_protocol::{
    Artifact, ElementDetail, ElementSummary, Multiplicity, Relationship,
};

/// Starting size of the query response buffer. Responses larger than this cost one extra
/// round trip, in which the host reports the size it needs.
#[cfg(target_arch = "wasm32")]
const INITIAL_RESPONSE_BYTES: usize = 64 * 1024;

pub trait Guest {
    fn generate(args: Vec<String>) -> Result<Vec<Artifact>, String>;
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "spec42")]
unsafe extern "C" {
    fn query(
        operation: i32,
        request_ptr: i32,
        request_len: i32,
        response_ptr: i32,
        response_capacity: i32,
    ) -> i64;
    fn diagnostic(
        level: i32,
        message_ptr: i32,
        message_len: i32,
        element_ptr: i32,
        element_len: i32,
    );
}

const fn str_eq(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

// `#[link(wasm_import_module = ...)]` needs a string literal, so the namespace cannot be
// written as `protocol::IMPORT_MODULE` directly. Fail the build if the two ever disagree.
const _: () = assert!(
    str_eq(protocol::IMPORT_MODULE, "spec42"),
    "the linked import module must match protocol::IMPORT_MODULE"
);

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// Reused across queries: a fresh `vec![0; 64 KiB]` per call spends most of a
    /// generator's time zeroing a buffer that is immediately overwritten.
    static RESPONSE_SCRATCH: core::cell::RefCell<Vec<u8>> =
        const { core::cell::RefCell::new(Vec::new()) };
}

#[cfg(target_arch = "wasm32")]
fn call_query<T: DeserializeOwned>(operation: i32, request: &impl Serialize) -> Result<T, String> {
    let request = postcard::to_allocvec(request).map_err(|error| error.to_string())?;
    RESPONSE_SCRATCH.with(|scratch| {
        // `call_query` never re-enters itself: the host makes no callbacks into the guest.
        let mut response = scratch.borrow_mut();
        if response.len() < INITIAL_RESPONSE_BYTES {
            response.resize(INITIAL_RESPONSE_BYTES, 0);
        }
        loop {
            let status = unsafe {
                query(
                    operation,
                    request.as_ptr() as i32,
                    request.len() as i32,
                    response.as_mut_ptr() as i32,
                    response.len() as i32,
                )
            };
            if status < 0 {
                let required = usize::try_from(-status)
                    .map_err(|_| "host returned an invalid response size".to_owned())?;
                response.resize(required, 0);
                continue;
            }
            let length = usize::try_from(status)
                .map_err(|_| "host returned an invalid response length".to_owned())?;
            if length > response.len() {
                return Err("host response exceeded the supplied buffer".to_owned());
            }
            // Reject leftovers rather than ignoring them. Postcard is positional, so a host
            // that encodes a field this guest does not know about leaves bytes behind; every
            // later field would decode from the wrong offset and still look valid.
            let (value, rest) = postcard::take_from_bytes::<Result<T, String>>(&response[..length])
                .map_err(|error| format!("invalid response from Spec42: {error}"))?;
            if !rest.is_empty() {
                return Err(
                    "Spec42 response contained trailing bytes; host and guest disagree about \
                     the wire schema"
                        .to_owned(),
                );
            }
            return value;
        }
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn call_query<T: DeserializeOwned>(_: i32, _: &impl Serialize) -> Result<T, String> {
    panic!("Spec42 generator queries are only available in WebAssembly guests")
}

pub mod model {
    pub use spec42_generator_protocol::{
        ElementDetail, ElementSummary, ModelInfo, Multiplicity, Relationship, SourceRange,
    };

    use super::call_query;
    use spec42_generator_protocol::operation;

    pub fn info() -> Result<ModelInfo, String> {
        call_query(operation::INFO, &())
    }

    pub fn roots() -> Result<Vec<ElementSummary>, String> {
        call_query(operation::ROOTS, &())
    }

    pub fn find(metaclass: Option<&str>) -> Result<Vec<ElementSummary>, String> {
        call_query(operation::FIND, &metaclass)
    }

    pub fn children(owner: &str) -> Result<Vec<ElementSummary>, String> {
        call_query(operation::CHILDREN, &owner)
    }

    pub fn element(handle: &str) -> Result<ElementDetail, String> {
        call_query(operation::ELEMENT, &handle)
    }

    pub fn typed_by(feature: &str) -> Result<Option<ElementSummary>, String> {
        call_query(operation::TYPED_BY, &feature)
    }

    pub fn relationships(element: &str) -> Result<Vec<Relationship>, String> {
        call_query(operation::RELATIONSHIPS, &element)
    }

    pub fn effective_features(element: &str) -> Result<Vec<ElementSummary>, String> {
        call_query(operation::EFFECTIVE_FEATURES, &element)
    }
}

pub mod diagnostics {
    pub use spec42_generator_protocol::Level;

    #[cfg(target_arch = "wasm32")]
    pub fn log(level: Level, message: &str) {
        report(level, message, None)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn log(_: Level, _: &str) {
        panic!("Spec42 diagnostics are only available in WebAssembly guests")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn report(level: Level, message: &str, element: Option<&str>) {
        let (element_ptr, element_len) = element
            .map(|value| (value.as_ptr() as i32, value.len() as i32))
            .unwrap_or((0, 0));
        unsafe {
            super::diagnostic(
                level as i32,
                message.as_ptr() as i32,
                message.len() as i32,
                element_ptr,
                element_len,
            )
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn report(_: Level, _: &str, _: Option<&str>) {
        panic!("Spec42 diagnostics are only available in WebAssembly guests")
    }
}

#[doc(hidden)]
pub fn run_guest<T: Guest>(args_ptr: i32, args_len: i32) -> u64 {
    // The host allocated this buffer through `spec42_alloc` and hands ownership over; it
    // never frees it. `args_len` is exactly the length that was allocated.
    let input = unsafe {
        Box::from_raw(core::ptr::slice_from_raw_parts_mut(
            args_ptr as *mut u8,
            args_len as usize,
        ))
    };
    let result = postcard::from_bytes::<Vec<String>>(&input)
        .map_err(|error| format!("invalid generator arguments: {error}"))
        .and_then(T::generate);
    let output = postcard::to_allocvec(&result)
        .unwrap_or_else(|error| {
            postcard::to_allocvec(&Err::<Vec<Artifact>, _>(error.to_string())).unwrap()
        })
        .into_boxed_slice();
    let length = output.len() as u64;
    // Deliberately leaked: the host reads these bytes and then discards the whole store, so
    // there is nothing to free and no window in which the guest could free it safely.
    let pointer = Box::into_raw(output) as *mut u8 as u64;
    (length << 32) | pointer
}

#[macro_export]
macro_rules! export {
    ($guest:ty) => {
        /// Structural fingerprint of the wire schema this guest was built against. The host
        /// refuses to run a module whose fingerprint differs from its own.
        #[unsafe(no_mangle)]
        pub extern "C" fn spec42_abi_version() -> i64 {
            $crate::protocol::SCHEMA_FINGERPRINT as i64
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn spec42_alloc(length: i32) -> i32 {
            let allocation = vec![0_u8; length as usize].into_boxed_slice();
            Box::into_raw(allocation) as *mut u8 as i32
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn spec42_generate(args_ptr: i32, args_len: i32) -> u64 {
            $crate::run_guest::<$guest>(args_ptr, args_len)
        }
    };
}
