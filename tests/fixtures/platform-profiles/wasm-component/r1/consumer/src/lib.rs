//! Controlled WebAssembly component revision 1 semantics.

pub fn normalize(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::normalize;

    #[test]
    fn normalizes_owner_input() {
        assert_eq!(normalize("  READY  "), "ready");
    }
}
