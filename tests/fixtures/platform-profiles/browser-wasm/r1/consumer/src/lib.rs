//! Controlled browser-WASM revision 1 fixture.

const MAX_TEXT_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderError {
    TextTooLong,
}

/// Renders escaped caller text as one status paragraph.
pub fn render_status(text: &str) -> Result<String, RenderError> {
    if text.len() > MAX_TEXT_BYTES {
        return Err(RenderError::TextTooLong);
    }
    let mut output = String::from("<p>");
    escape_into(text, &mut output);
    output.push_str("</p>");
    Ok(output)
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
    fn escapes_injection_shaped_text() {
        assert_eq!(
            render_status("<script a='b'>&\"").unwrap(),
            "<p>&lt;script a=&#39;b&#39;&gt;&amp;&quot;</p>"
        );
    }

    #[test]
    fn rejects_oversized_text() {
        assert_eq!(
            render_status(&"x".repeat(129)),
            Err(RenderError::TextTooLong)
        );
    }
}
