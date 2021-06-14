use super::{Inline};

/// Parse a string slice into a vector of
/// all the inline elements found inside.
pub fn parse_inline<'a>(source: &'a str) -> Vec<Inline> {
    vec![Inline::Text(source.into())]
}