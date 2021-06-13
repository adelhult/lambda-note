use lazy_static::lazy_static;
use regex::Regex;
use std::str;

use super::{Inline};


/// Parse a string slice into a vector of
/// all the inline elements found inside.
pub fn parse_inline<'a>(source: &'a str) -> Vec<Inline> {
    if source.is_empty() {
        return vec![];
    }

    lazy_static! {
        static ref INLINE_RULE: Regex = Regex::new(
            r"(?x)
            # Literals and escape characters
            (?P<escape>\\(?:
                /|\*|=|\^|\+|_|
                Lambda|lambda
                |Alpha|alpha
            ))
            
            # Typography tags
            | /(?P<italic>\S|\S.*?\S)/
            | \*(?P<bold>\S|\S.*?\S)\*
            | =(?P<underline>\S|\S.*?\S)=
            | \^(?P<superscript>\S|\S.*?\S)\^
            | _(?P<subscript>\S|\S.*?\S)_
            | \+(?P<strikethrough>\S|\S.*?\S)\+
            
            # Extension
            | \|\s*(P?<ident>\w+?)\s*(?:,(?P<args>[^|]+))*\|
        ",
        )
        .unwrap();
        static ref INLINE_EXTENSION: Regex =
            Regex::new(r"\|\s*(P?<ident>\w+?)\s*(?:,(?P<args>[^|]+))*\|").unwrap();
    }

    if let Some(m) = INLINE_RULE.find(source) {
        let mut result = vec![];

        let preceding_text = &source[..m.start()];
        let symbol = &source[m.start()..m.start() + 1];

        // add the preceding text
        if !preceding_text.is_empty() {
            result.push(Inline::Text(preceding_text.into()));
        }

        // calc the remainder slice
        let remainder = if m.end() >= source.len() {
            ""
        } else if symbol == r"\" {
            // escaped items does not have a end tag
            &source[m.end()..]
        } else {
            &source[(m.end() + 1)..]
        };

        // helper function: trim the ends of matches
        let trim = |s: &'a str, m: regex::Match, left, right| -> &'a str {
            &s[(m.start() + left)..(m.end() - right)]
        };

        // avoids boilerplate code when returning tags
        macro_rules! tag {
            ($i:ident) => {
                vec![Inline::$i(parse_inline(trim(source, m, 1, 1)))]
            };
        }

        // parse the children and add the node to the result vector
        result.append(&mut match symbol {
            "*" => tag!(Bold),
            "/" => tag!(Italic),
            "=" => tag!(Underline),
            "^" => tag!(Superscript),
            "_" => tag!(Subscript),
            "+" => tag!(Strikethrough),
            r"\" => vec![Inline::Escaped(trim(source, m, 1, 0).into())],
            "|" => {
                // This is an extension. Not  the cleanest code,
                // but we will just capture the groups with a repeated regex match
                let contents = trim(source, m, 1, 1);
                vec![] // todo
            }
            _ => panic!("inline regex error, unknown tag symbol!"),
        });

        // parse the remainder
        result.append(&mut parse_inline(remainder));

        result
    } else {
        vec![Inline::Text(source.into())]
    }
}