use pulldown_cmark::{
    Event, Parser, Tag,
    TagEnd::{self},
};

use crate::{engine::converter::Converter, errors::CambiarErrors, formats::FileFormats};

pub struct MdtoTxt;

impl Converter for MdtoTxt {
    fn input_format(&self) -> crate::formats::FileFormats {
        FileFormats::Md
    }

    fn output_format(&self) -> crate::formats::FileFormats {
        FileFormats::Txt
    }

    fn convert(
        &self,
        input: &std::path::Path,
        output: &std::path::Path,
    ) -> Result<(), crate::errors::CambiarErrors> {
        let mut output_string = String::new();

        let markdown = std::fs::read_to_string(input).map_err(|_| CambiarErrors::ReadError)?;

        let parser = Parser::new(&markdown);

        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Item => output_string.push_str("• "),
                    _ => {}
                },

                Event::End(tag) => match tag {
                    TagEnd::Paragraph => output_string.push_str("\n\n"),
                    TagEnd::Heading(_) => output_string.push_str("\n\n"),
                    TagEnd::Item => output_string.push('\n'),
                    _ => {}
                },

                Event::TaskListMarker(checked) => {
                    if checked {
                        output_string.push_str("[x] ");
                    } else {
                        output_string.push_str("[ ] ");
                    }
                }
                pulldown_cmark::Event::Text(cow_str) => output_string.push_str(&cow_str),
                pulldown_cmark::Event::Code(cow_str) => output_string.push_str(&cow_str),
                pulldown_cmark::Event::InlineMath(cow_str) => output_string.push_str(&cow_str),
                pulldown_cmark::Event::DisplayMath(cow_str) => output_string.push_str(&cow_str),
                pulldown_cmark::Event::Html(_) => {}
                pulldown_cmark::Event::InlineHtml(_) => {}
                pulldown_cmark::Event::SoftBreak => output_string.push_str("\n"),
                pulldown_cmark::Event::HardBreak => output_string.push_str("\n"),
                pulldown_cmark::Event::Rule => output_string.push_str("------------------"),
                _ => {}
            }
        }

        std::fs::write(output, output_string).map_err(|_| CambiarErrors::WriteError)?;

        Ok(())
    }
}
