//! Controlled WebAssembly component revision 2 semantics.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizeError {
    InvalidCharacter,
    TooLong,
}

pub fn normalize(input: &str) -> Result<String, NormalizeError> {
    if input.len() > 32 {
        return Err(NormalizeError::TooLong);
    }
    if !input.is_ascii() {
        return Err(NormalizeError::InvalidCharacter);
    }
    Ok(input.trim().to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::{NormalizeError, normalize};

    #[test]
    fn normalizes_and_rejects_exactly() {
        assert_eq!(normalize(" READY "), Ok("ready".to_owned()));
        assert_eq!(normalize("cafe\u{301}"), Err(NormalizeError::InvalidCharacter));
        assert_eq!(normalize(&"x".repeat(33)), Err(NormalizeError::TooLong));
    }
}
