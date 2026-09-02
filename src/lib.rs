//! Small, dependency-free Markdown rendering primitives for Cedar.

/// Render a minimal Markdown document as HTML.
pub fn render(markdown: &str) -> String {
    markdown
        .lines()
        .map(|line| {
            if let Some(text) = line.strip_prefix("# ") {
                format!("<h1>{}</h1>", escape(text))
            } else if let Some(text) = line.strip_prefix("## ") {
                format!("<h2>{}</h2>", escape(text))
            } else if line.trim().is_empty() {
                String::new()
            } else {
                format!("<p>{}</p>", escape(line))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn renders_headings_and_paragraphs() {
        assert_eq!(render("# Cedar\n\nHello"), "<h1>Cedar</h1>\n\n<p>Hello</p>");
    }

    #[test]
    fn escapes_html_in_text() {
        assert_eq!(render("<script>"), "<p>&lt;script&gt;</p>");
    }
}
