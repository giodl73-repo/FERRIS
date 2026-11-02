use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

pub const MAX_INPUT_BYTES: usize = 1024 * 1024;
pub const MAX_DEPTH: usize = 24;
pub const MAX_PATH_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_REPORT_BYTES: usize = 4 * 1024 * 1024;

fn refusal(code: &str, message: &str) -> String {
    serde_json::to_string_pretty(&json!({
        "schema": "ferris.browser-admission/v1",
        "adapter_error": {"code": code, "message": message},
        "scope": "Browser resource admission only. No native comparison result or approval is claimed.",
    })).expect("fixed admission report serializes")
}

// Resource scan only: malformed JSON is still classified by the existing core.
fn exceeds_depth(text: &str) -> bool {
    let (mut depth, mut quoted, mut escaped) = (0usize, false, false);
    for byte in text.bytes() {
        if quoted {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quoted = false;
            }
        } else {
            match byte {
                b'"' => quoted = true,
                b'{' | b'[' => {
                    depth += 1;
                    if depth > MAX_DEPTH {
                        return true;
                    }
                }
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => (),
            }
        }
    }
    false
}
fn path_budget(value: &Value, path_len: usize, total: &mut usize) -> bool {
    match value {
        Value::Object(values) if !values.is_empty() => {
            for (key, value) in values {
                let escaped_len = key
                    .bytes()
                    .map(|b| if b == b'~' || b == b'/' { 2 } else { 1 })
                    .sum::<usize>();
                if !path_budget(value, path_len + 1 + escaped_len, total) {
                    return false;
                }
            }
        }
        Value::Array(values) if !values.is_empty() => {
            for (index, value) in values.iter().enumerate() {
                if !path_budget(value, path_len + 1 + index.to_string().len(), total) {
                    return false;
                }
            }
        }
        _ => {
            *total = total.saturating_add(path_len);
        }
    }
    *total <= MAX_PATH_BYTES
}

/// Reuses native parsing/diff results for admitted input. Resource refusals use
/// a separate schema and never masquerade as a native command result.
#[wasm_bindgen]
pub fn compare_profiles(before: &str, after: &str) -> String {
    // The unchanged core owns oversized-input diagnostics and short circuits
    // comparison when either input cannot be admitted completely.
    if before.len() <= MAX_INPUT_BYTES && after.len() <= MAX_INPUT_BYTES {
        let mut total = 0usize;
        for text in [before, after] {
            if exceeds_depth(text) {
                return refusal(
                    "FERRIS-BROWSER-DEPTH",
                    "Browser input nesting exceeds 24 containers; input is retained.",
                );
            }
            // This secondary parse estimates resource use only. Its normalized
            // value is never passed to the core or used as semantic evidence.
            if let Ok(value) = serde_json::from_str::<Value>(text)
                && !path_budget(&value, 0, &mut total)
            {
                return refusal(
                    "FERRIS-BROWSER-PATH-BUDGET",
                    "Combined input leaf-path budget exceeds 2 MiB; no partial comparison is emitted.",
                );
            }
        }
    }
    let record = ferris_core::create_profile_diff_from_bytes(before.as_bytes(), after.as_bytes());
    let text = serde_json::to_string_pretty(&record).expect("typed profile results serialize");
    if text.len() > MAX_REPORT_BYTES {
        return refusal(
            "FERRIS-BROWSER-REPORT-BUDGET",
            "Native report exceeds the browser's 4 MiB output budget; no partial report is exported.",
        );
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    const BEFORE: &str =
        include_str!("../../../tests/fixtures/profile-evidence/pure-data/before.json");
    fn parse(text: &str) -> Value {
        serde_json::from_str(text).unwrap()
    }
    #[test]
    fn admitted_inputs_keep_exact_core_envelope() {
        assert_eq!(
            compare_profiles(BEFORE, BEFORE),
            serde_json::to_string_pretty(&ferris_core::create_profile_diff_from_bytes(
                BEFORE.as_bytes(),
                BEFORE.as_bytes()
            ))
            .unwrap()
        );
    }
    #[test]
    fn depth_guard_ignores_brackets_and_escapes_inside_strings() {
        let mut v = parse(BEFORE);
        v["sections"]["features"] = json!({"literal": "[[[[{{{{\"quoted\"}}}}]]]]"});
        assert!(
            parse(&compare_profiles(BEFORE, &v.to_string()))
                .get("adapter_error")
                .is_none()
        );
        for _ in 0..MAX_DEPTH {
            v["sections"]["features"] = json!([v["sections"]["features"].clone()]);
        }
        assert_eq!(
            parse(&compare_profiles(BEFORE, &v.to_string()))["adapter_error"]["code"],
            "FERRIS-BROWSER-DEPTH"
        );
    }
    #[test]
    fn path_amplification_refuses_before_native_comparison() {
        let mut v = parse(BEFORE);
        let mut tree = json!(vec![0; 1000]);
        for _ in 0..12 {
            tree = json!({"x".repeat(200):tree});
        }
        v["sections"]["features"] = tree;
        assert_eq!(
            parse(&compare_profiles(BEFORE, &v.to_string()))["adapter_error"]["code"],
            "FERRIS-BROWSER-PATH-BUDGET"
        );
    }
    #[test]
    fn malformed_and_oversized_inputs_keep_native_diagnostics() {
        for text in [
            "{".to_owned(),
            " ".repeat(MAX_INPUT_BYTES + 1),
            "{\"x\":1,\"x\":2}".to_owned(),
        ] {
            let expected =
                ferris_core::create_profile_diff_from_bytes(text.as_bytes(), BEFORE.as_bytes());
            assert_eq!(
                compare_profiles(&text, BEFORE),
                serde_json::to_string_pretty(&expected).unwrap()
            );
        }
    }
}
