use wasm_bindgen::prelude::*;

/// Pure profile comparison. The existing Rust core owns all parsing and results.
#[wasm_bindgen]
pub fn compare_profiles(before: &str, after: &str) -> String {
    serde_json::to_string_pretty(&ferris_core::create_profile_diff_from_bytes(
        before.as_bytes(),
        after.as_bytes(),
    ))
    .expect("typed profile results serialize")
}
