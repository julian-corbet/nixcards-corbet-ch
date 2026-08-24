use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, html};

pub fn contains_raw_html(markdown: &str) -> bool {
    Parser::new_ext(markdown, Options::all())
        .any(|event| matches!(event, Event::Html(_) | Event::InlineHtml(_)))
}

pub fn render_markdown_safe(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all()).filter_map(|event| match event {
        Event::Html(content) | Event::InlineHtml(content) => Some(Event::Text(content)),
        Event::Start(Tag::Link { .. })
        | Event::End(TagEnd::Link)
        | Event::Start(Tag::Image { .. })
        | Event::End(TagEnd::Image) => None,
        event => Some(event),
    });
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub fn markdown_to_plain_text(markdown: &str) -> String {
    let mut output = String::new();
    for event in Parser::new_ext(markdown, Options::all()) {
        match event {
            Event::Text(text) | Event::Code(text) => output.push_str(&text),
            Event::SoftBreak | Event::HardBreak => output.push('\n'),
            Event::End(
                TagEnd::Paragraph
                | TagEnd::Heading(_)
                | TagEnd::Item
                | TagEnd::CodeBlock
                | TagEnd::TableRow,
            ) => output.push('\n'),
            _ => {}
        }
    }
    output.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_html_is_escaped_and_links_are_plain_text() {
        let rendered =
            render_markdown_safe("<script>alert(1)</script> [open](javascript:alert(1))");
        assert!(!rendered.contains("<script>"));
        assert!(!rendered.contains("href="));
        assert!(rendered.contains("&lt;script&gt;"));
        assert!(rendered.contains("open"));
    }
}
