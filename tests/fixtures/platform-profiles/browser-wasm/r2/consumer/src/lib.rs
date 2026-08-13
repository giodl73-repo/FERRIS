//! Controlled browser-WASM revision 2 fixture.

const MAX_TEXT_BYTES: usize = 128;
const MAX_LANGUAGE_BYTES: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderError {
    InvalidLanguage,
    TextTooLong,
}

/// Renders escaped caller text with explicit language and live-region metadata.
pub fn render_status(language: &str, text: &str) -> Result<String, RenderError> {
    if !valid_language(language) {
        return Err(RenderError::InvalidLanguage);
    }
    if text.len() > MAX_TEXT_BYTES {
        return Err(RenderError::TextTooLong);
    }
    let mut output = String::from("<p lang=\"");
    output.push_str(language);
    output.push_str("\" aria-live=\"polite\">");
    escape_into(text, &mut output);
    output.push_str("</p>");
    Ok(output)
}

fn valid_language(language: &str) -> bool {
    !language.is_empty()
        && language.len() <= MAX_LANGUAGE_BYTES
        && !language.starts_with('-')
        && !language.ends_with('-')
        && language
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        && !language.as_bytes().windows(2).any(|pair| pair == b"--")
}

fn escape_into(text: &str, output: &mut String) {
    for character in text.chars() {
        output.push_str(match character {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&#39;",
            _ => {
                output.push(character);
                continue;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{RenderError, render_status};

    #[test]
    fn renders_accessible_escaped_status() {
        assert_eq!(
            render_status("en-us", "<ready>").unwrap(),
            "<p lang=\"en-us\" aria-live=\"polite\">&lt;ready&gt;</p>"
        );
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            render_status("EN", "ready"),
            Err(RenderError::InvalidLanguage)
        );
        assert_eq!(
            render_status("en", &"x".repeat(129)),
            Err(RenderError::TextTooLong)
        );
    }
}
