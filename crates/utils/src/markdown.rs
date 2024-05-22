/// GitHub Flavored Markdown to HTML
/// this function never errors with normal markdown because markdown does not have syntax errors.
#[must_use]
pub fn markdown_to_html(value: &str) -> String {
    match markdown::to_html_with_options(value, &markdown::Options::gfm()) {
        Ok(result) => result,
        Err(result) => result.to_string(),
    }
}
