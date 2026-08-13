//! Controlled pure-data revision 1 fixture.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizeError {
    Empty,
    NonAscii,
    InternalWhitespace,
}

/// Normalizes one ASCII record key.
///
/// ```
/// use ferris_profile_pure_data::normalize_key;
///
/// assert_eq!(normalize_key("  Account-ID  ").unwrap(), "account-id");
/// ```
pub fn normalize_key(input: &str) -> Result<String, NormalizeError> {
    let trimmed = input.trim_matches(|character: char| character.is_ascii_whitespace());
    if trimmed.is_empty() {
        return Err(NormalizeError::Empty);
    }
    if !trimmed.is_ascii() {
        return Err(NormalizeError::NonAscii);
    }
    if trimmed.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(NormalizeError::InternalWhitespace);
    }
    Ok(trimmed.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::{NormalizeError, normalize_key};

    #[test]
    fn normalizes_ascii_key() {
        assert_eq!(normalize_key("  Account-ID  "), Ok("account-id".to_owned()));
    }

    #[test]
    fn rejects_internal_whitespace() {
        assert_eq!(
            normalize_key("Account ID"),
            Err(NormalizeError::InternalWhitespace)
        );
    }

    #[test]
    fn rejects_empty_and_non_ascii_input() {
        assert_eq!(normalize_key(" \t "), Err(NormalizeError::Empty));
        assert_eq!(normalize_key("cafe\u{301}"), Err(NormalizeError::NonAscii));
    }
}
