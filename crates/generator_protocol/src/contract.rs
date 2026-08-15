//! The ABI contract, declared once.
//!
//! Operations, diagnostic levels, their numeric wire codes, the versions and the
//! compatibility token all come from the single [`abi_contract!`] invocation at the bottom of
//! this file. Everything else — the enums, their `TryFrom<i32>`, the tables mixed into the
//! token, the checked-in `generator-abi.json` manifest and the tables in the ABI
//! documentation — is derived from it.
//!
//! The point is that divergence becomes impossible rather than merely tested for. Every
//! previous round of review found a contract element that lived in two places and could drift:
//! operation codes outside the schema hash, diagnostic level discriminants outside the token,
//! a semantic version declared twice. Those were fixed individually and each fix added a
//! completeness test. A single source removes the class of bug and the tests with it; the
//! pinned-token test that remains does not check for drift, it announces a deliberate change.

use core::fmt;

/// Declares the executable ABI: the numbers, names and versions that cross the boundary.
///
/// Generates for each enum: the type with explicit discriminants, an `ALL` table of
/// `(name, code)` used by the compatibility token, `code()`, and a checked `try_from_code`.
/// Generates for the contract as a whole: the version constants and a manifest renderer.
macro_rules! abi_contract {
    (
        abi_version: $abi_version:expr,
        semantic_api_version: $semantic_version:expr,
        namespace: $namespace:expr,
        import_module: $import_module:expr,

        $(#[$op_meta:meta])*
        operations {
            $( $(#[doc = $op_doc:expr])* $op:ident = $op_code:expr => ($op_request:ty, $op_response:ty) ),* $(,)?
        }

        $(#[$level_meta:meta])*
        levels {
            $( $(#[doc = $level_doc:expr])* $level:ident = $level_code:expr ),* $(,)?
        }
    ) => {
        /// Version of the executable ABI. Mixed into the compatibility token, so bumping it
        /// alone is enough to declare an intentional break.
        pub const ABI_VERSION: u32 = $abi_version;

        /// Version of the semantic contract: result ordering, defaulting, and what each query
        /// means. Bump it when observable behaviour changes with no type change at all.
        ///
        /// Declared here and re-exported wherever else it is needed, so the value feeding the
        /// compatibility token and the one reported through `model.info` cannot differ.
        pub const SEMANTIC_API_VERSION: &str = $semantic_version;

        /// Namespace mixed into the schema hash and the token.
        pub const SCHEMA_PATH: &str = $namespace;

        /// WebAssembly import module the two host functions live in.
        pub const IMPORT_MODULE: &str = $import_module;

        $(#[$op_meta])*
        #[repr(i32)]
        pub enum Operation {
            $( $(#[doc = $op_doc])* $op = $op_code ),*
        }

        impl Operation {
            /// Every operation with the code it is transmitted as, and the Postcard request
            /// and response types it carries.
            ///
            /// Mixed into [`COMPATIBILITY_TOKEN`]: operation codes are plain integers that no
            /// type-level schema hash can see, so renumbering one would otherwise route an old
            /// guest's `children` request to `element` with every type identical.
            pub const ALL: &'static [OperationSpec] = &[
                $( OperationSpec {
                    name: stringify!($op),
                    code: $op_code,
                    // Rendered from the same type tokens the marker types use, so a
                    // description cannot drift from the type it describes.
                    // Rendered from the same type tokens the marker types use, so a
                    // description cannot drift from the type it describes.
                    request: stringify!($op_request),
                    response: stringify!($op_response),
                } ),*
            ];

            pub const fn code(self) -> i32 {
                self as i32
            }

            /// Resolves a wire code, rejecting anything this ABI does not define.
            pub fn try_from_code(code: i32) -> Result<Self, UnknownCode> {
                match code {
                    $( $op_code => Ok(Self::$op), )*
                    other => Err(UnknownCode { kind: "query operation", code: other }),
                }
            }
        }

        $(#[$level_meta])*
        #[repr(i32)]
        pub enum Level {
            $( $(#[doc = $level_doc])* $level = $level_code ),*
        }

        impl Level {
            /// Every level with the code it is transmitted as.
            ///
            /// Levels cross as raw integers rather than through Postcard, so their
            /// discriminants are contract too and are equally invisible to the schema hash.
            pub const ALL: &'static [(&'static str, i32)] =
                &[ $( (stringify!($level), $level_code) ),* ];

            pub const fn code(self) -> i32 {
                self as i32
            }

            pub fn try_from_code(code: i32) -> Result<Self, UnknownCode> {
                match code {
                    $( $level_code => Ok(Self::$level), )*
                    other => Err(UnknownCode { kind: "diagnostic level", code: other }),
                }
            }
        }

        /// One marker type per operation, binding its code to its payload types.
        ///
        /// Host decoding and SDK calls both go through these, so the request and response
        /// types are chosen once. Previously the declaration named them as strings while the
        /// host and SDK independently picked actual types: changing `Find` from
        /// `Option<String>` to `String` on both sides would have left the token unchanged,
        /// because both types already appear elsewhere in the wire schema.
        pub mod query {
            use super::{Operation, Query};
            #[allow(unused_imports)]
            use crate::*;

            $(
                $(#[doc = $op_doc])*
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct $op;

                impl Query for $op {
                    type Request = $op_request;
                    type Response = $op_response;
                    const OPERATION: Operation = Operation::$op;
                }
            )*
        }

        /// Renders the contract as JSON.
        ///
        /// Written by hand rather than with `serde_json` so the guest-facing crate keeps its
        /// two dependencies, and so the byte layout is fixed by this function rather than by a
        /// serializer's formatting choices. The result is checked in as
        /// `docs/generation/generator-abi.json`, which makes every token change arrive as a
        /// reviewable diff naming exactly what moved.
        pub fn contract_manifest() -> String {
            let mut out = String::new();
            out.push_str("{\n");
            out.push_str(&format!("  \"abiVersion\": {},\n", ABI_VERSION));
            out.push_str(&format!(
                "  \"semanticApiVersion\": \"{}\",\n",
                SEMANTIC_API_VERSION
            ));
            out.push_str(&format!("  \"namespace\": \"{}\",\n", SCHEMA_PATH));
            out.push_str(&format!("  \"importModule\": \"{}\",\n", IMPORT_MODULE));
            out.push_str(&format!(
                "  \"schemaFingerprint\": \"{:#018x}\",\n",
                crate::SCHEMA_FINGERPRINT
            ));
            out.push_str(&format!(
                "  \"compatibilityToken\": \"{:#018x}\",\n",
                COMPATIBILITY_TOKEN
            ));

            out.push_str("  \"operations\": [\n");
            for (index, spec) in Operation::ALL.iter().enumerate() {
                out.push_str(&format!(
                    "    {{ \"name\": \"{}\", \"code\": {}, \"request\": \"{}\", \"response\": \"{}\" }}{}\n",
                    spec.name,
                    spec.code,
                    spec.request,
                    spec.response,
                    if index + 1 == Operation::ALL.len() { "" } else { "," }
                ));
            }
            out.push_str("  ],\n");

            out.push_str("  \"diagnosticLevels\": [\n");
            for (index, (name, code)) in Level::ALL.iter().enumerate() {
                out.push_str(&format!(
                    "    {{ \"name\": \"{}\", \"code\": {} }}{}\n",
                    name,
                    code,
                    if index + 1 == Level::ALL.len() { "" } else { "," }
                ));
            }
            out.push_str("  ]\n");
            out.push_str("}\n");
            out
        }

        /// Compile-time contract digest guests report and the host compares.
        pub const COMPATIBILITY_TOKEN: u64 = compute_compatibility_token();

        const fn compute_compatibility_token() -> u64 {
            const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
            const PRIME: u64 = 0x0000_0100_0000_01b3;

            const fn mix_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
                let mut index = 0;
                while index < bytes.len() {
                    hash ^= bytes[index] as u64;
                    hash = hash.wrapping_mul(PRIME);
                    index += 1;
                }
                hash
            }

            const fn mix_u64(hash: u64, value: u64) -> u64 {
                mix_bytes(hash, &value.to_le_bytes())
            }

            let mut hash = OFFSET;
            hash = mix_bytes(hash, SCHEMA_PATH.as_bytes());
            hash = mix_bytes(hash, IMPORT_MODULE.as_bytes());
            hash = mix_u64(hash, ABI_VERSION as u64);
            hash = mix_u64(hash, crate::SCHEMA_FINGERPRINT);
            hash = mix_bytes(hash, SEMANTIC_API_VERSION.as_bytes());

            let mut index = 0;
            while index < Operation::ALL.len() {
                let spec = &Operation::ALL[index];
                hash = mix_bytes(hash, spec.name.as_bytes());
                hash = mix_u64(hash, spec.code as u64);
                // Request and response shapes are contract as much as the codes: swapping two
                // operations' payload types would leave names and numbers untouched.
                hash = mix_bytes(hash, spec.request.as_bytes());
                hash = mix_bytes(hash, spec.response.as_bytes());
                index += 1;
            }

            let mut index = 0;
            while index < Level::ALL.len() {
                let (name, code) = Level::ALL[index];
                hash = mix_bytes(hash, name.as_bytes());
                hash = mix_u64(hash, code as u64);
                index += 1;
            }
            hash
        }
    };
}

/// Binds an operation to the types it carries.
///
/// Implemented by the marker types in [`query`], generated from the same declaration that
/// produces the operation codes, so a payload type and its wire code cannot be chosen
/// independently.
pub trait Query {
    type Request: serde::Serialize + serde::de::DeserializeOwned;
    type Response: serde::Serialize + serde::de::DeserializeOwned;
    const OPERATION: Operation;
}

/// One operation's full declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationSpec {
    pub name: &'static str,
    pub code: i32,
    /// Postcard request type, as written in the specification.
    pub request: &'static str,
    /// Postcard response type, inside the `Result<T, String>` every response carries.
    pub response: &'static str,
}

/// A wire code this ABI does not define.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownCode {
    pub kind: &'static str,
    pub code: i32,
}

impl fmt::Display for UnknownCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown Spec42 {} {}", self.kind, self.code)
    }
}

impl std::error::Error for UnknownCode {}

abi_contract! {
    abi_version: 4,
    semantic_api_version: "0.1.0",
    namespace: "spec42:generator-abi/4",
    import_module: "spec42",

    /// A read-only semantic query.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    operations {
        /// Snapshot provenance and versions.
        Info = 0 => ((), ModelInfo),
        /// Elements with no owner in the projection.
        Roots = 1 => ((), Vec<ElementSummary>),
        /// Every element, or those of one metaclass. `None` means every element.
        Find = 2 => (Option<String>, Vec<ElementSummary>),
        /// Direct children of an element.
        Children = 3 => (String, Vec<ElementSummary>),
        /// Full detail for one element.
        Element = 4 => (String, ElementDetail),
        /// The type of a feature, if it has one.
        TypedBy = 5 => (String, Option<ElementSummary>),
        /// Outgoing relationships of an element.
        Relationships = 6 => (String, Vec<Relationship>),
        /// Direct features first, then inherited nearest-first, with shadowing applied.
        EffectiveFeatures = 7 => (String, Vec<ElementSummary>),
        /// Authoritative typing state for a requirement usage.
        RequirementTyping = 8 => (String, RequirementUsageTyping),
    }

    /// Severity of a generator diagnostic.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postcard_schema::Schema)]
    #[serde(rename_all = "lowercase")]
    levels {
        Debug = 0,
        Info = 1,
        Warning = 2,
        Error = 3,
    }
}
