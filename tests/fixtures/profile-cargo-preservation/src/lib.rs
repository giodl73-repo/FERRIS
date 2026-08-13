pub fn owner_behavior() -> &'static str {
    "cargo-owner-baseline"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_behavior_remains_available() {
        assert_eq!(owner_behavior(), "cargo-owner-baseline");
    }
}
