//! Regenerates the checked-in ABI manifest.
//!
//! An example rather than a binary so that guests depending on this crate never build it.
//!
//! ```sh
//! cargo run -p spec42-generator-protocol --example abi-manifest \
//!     > docs/generation/generator-abi.json
//! ```

fn main() {
    print!("{}", spec42_generator_protocol::contract_manifest());
}
