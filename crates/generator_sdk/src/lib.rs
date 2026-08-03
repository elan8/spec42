//! Rust guest SDK for Spec42's core WebAssembly generator ABI.

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use spec42_generator_protocol as protocol;
pub use spec42_generator_protocol::{
    Artifact, ElementDetail, ElementSummary, Multiplicity, Relationship,
};

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

#[cfg(target_arch = "wasm32")]
fn call_query<T: DeserializeOwned>(operation: i32, request: &impl Serialize) -> Result<T, String> {
    let request = postcard::to_allocvec(request).map_err(|error| error.to_string())?;
    let mut response = vec![0; 64 * 1024];
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
        return postcard::from_bytes::<Result<T, String>>(&response[..length])
            .map_err(|error| format!("invalid response from Spec42: {error}"))?;
    }
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

    pub fn info() -> ModelInfo {
        call_query(operation::INFO, &()).expect("Spec42 model info query failed")
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
        unsafe {
            super::diagnostic(
                level as i32,
                message.as_ptr() as i32,
                message.len() as i32,
                0,
                0,
            )
        }
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
    let pointer = Box::into_raw(output) as *mut u8 as u64;
    (length << 32) | pointer
}

#[macro_export]
macro_rules! export {
    ($guest:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn spec42_alloc(length: i32) -> i32 {
            let allocation = vec![0_u8; length as usize].into_boxed_slice();
            Box::into_raw(allocation) as *mut u8 as i32
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn spec42_dealloc(pointer: i32, length: i32) {
            unsafe {
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    pointer as *mut u8,
                    length as usize,
                )));
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn spec42_generate(args_ptr: i32, args_len: i32) -> u64 {
            $crate::run_guest::<$guest>(args_ptr, args_len)
        }
    };
}
