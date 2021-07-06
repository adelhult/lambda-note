use lazy_static::lazy_static;
use tst::{tstmap, TSTMap};

use super::{EscapeChars, Inline, Tag};
use std::{iter::Peekable, str::Chars};

struct ParserState<'a> {
    result: Vec<Inline>,
    stack: Vec<Tag>,
    text_buffer: String,
    chars: Peekable<Chars<'a>>,
}

impl<'a> ParserState<'a> {
    fn new(source: &'a str) -> Self {
        ParserState {
            result: vec![],
            stack: vec![],
            text_buffer: String::new(),
            chars: source.chars().peekable(),
        }
    }

    /// push the text buffer to the result vec
    fn push_buffer(&mut self) {
        if self.text_buffer.is_empty() {
            return;
        }
        self.result.push(Inline::Text(self.text_buffer.clone()));
        self.text_buffer.clear();
    }
}

/// Parse a string slice into a vector of
/// all the inline elements found inside.
pub fn parse_inline<'a>(source: &'a str) -> Vec<Inline> {
    let mut state = ParserState::new(source);

    while let Some(current) = state.chars.next() {
        match current {
            '\\' => escape(&mut state),
            _ => state.text_buffer.push(current),
        }
    }

    // finally, push the iflast bit of the buffer
    state.push_buffer();

    state.result
}

/// Handle potential escape characters and add them to the result vec
fn escape(state: &mut ParserState) {
    lazy_static! {
        // a prefix tree maping all the possible special escape characters
        static ref ESCAPE_TRIE: TSTMap<EscapeChars> = tstmap! {
            "Lambda" =>  EscapeChars::BigLambda,
            "lambda" => EscapeChars::SmallLambda,
            "alpha" => EscapeChars::SmallAlpha,
        };
    }
    let mut s = String::new();
    while let Some(next_char) = state.chars.next() {
        s.push(next_char);

        let nr_matches = ESCAPE_TRIE.prefix_iter(&s).count();

        if nr_matches > 1 {
            continue;
        } else if nr_matches == 1 {
            if let Some(escape_char) = ESCAPE_TRIE.get(&s) {
                state.push_buffer();
                state.result.push(Inline::Escaped(escape_char.to_owned()));
                return;
            } else {
                continue; // we have a match but must still collect more chars
            }
        }

        // if there are no matches add the chars that we collect so far
        // to the text buffer
        state.text_buffer.push_str(&format!("\\{}", s));
        return;
    }
}
