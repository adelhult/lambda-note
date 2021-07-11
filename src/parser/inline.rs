// TODO: this code is somewhat messy
// half the logic lives in the struct and the other half outside
// this should be cleaned up at some point.

use lazy_static::lazy_static;
use tst::{tstmap, TSTMap};

use super::{EscapeChar, Inline, Tag};
use std::{iter::Peekable, str::Chars};

struct ParserState<'a> {
    result: Vec<Inline>,
    // the stack is used to keep track of begin tags
    // that are yet to be closed by an end tag.
    open_tags: Vec<Tag>,
    text_buffer: String,
    chars: Peekable<Chars<'a>>,
    prev_is_whitespace: bool,
}

impl<'a> ParserState<'a> {
    fn new(source: &'a str) -> Self {
        ParserState {
            result: vec![],
            open_tags: vec![],
            text_buffer: String::new(),
            chars: source.chars().peekable(),
            prev_is_whitespace: false,
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

    /// peek in the chars iterator and check if the next
    /// char is a given character
    fn next_is(&mut self, c: char) -> bool {
        if let Some(next) = self.chars.peek() {
            *next == c
        } else {
            false
        }
    }

    fn is_closing_tag(&self, tag: &Tag) -> bool {
        if let Some(prev_tag) = self.open_tags.last() {
            prev_tag == tag
        } else {
            false
        }
    }

    /// corrects the result vec.
    /// It will empty the result buffer one last time
    /// and look at the stack
    /// of unclosed tags and correct based upon them.
    fn amend(&mut self) {
        self.push_buffer();

        // Fix missplaced Begin(Tag)'s
        self.result.reverse();

        let mut i = 0;
        while i < self.result.len() {
            if self.open_tags.is_empty() {
                break;
            }

            let missplaced_tag = self.open_tags.last().unwrap();

            if let Inline::Begin(tag) = &self.result[i] {
                if tag == missplaced_tag {
                    self.result[i] = Inline::Text(tag_to_string(&tag));
                    self.open_tags.pop();
                }
            }
            i += 1;
        }

        self.result.reverse();
    }
}

/// Parse a string slice into a vector of
/// all the inline elements found inside of it.
pub fn parse_inline<'a>(source: &'a str) -> Vec<Inline> {
    let mut state = ParserState::new(source);

    while let Some(current) = state.chars.next() {
        match current {
            '\\' => escape(&mut state),
            '|' => extension(&mut state),
            '*' | '/' | '=' | '_' | '^' | '~' => tag(current, &mut state),
            '\n' => {
                if state.chars.peek().is_some() {
                    state.text_buffer.push(' ');
                }
            }
            _ => state.text_buffer.push(current),
        }

        state.prev_is_whitespace = current.is_whitespace();
    }
    state.amend();
    state.result
}

/// Parse inline extensions
fn extension(state: &mut ParserState) {
    let mut content = String::new();
    let mut terminated = false;
    while let Some(c) = state.chars.next() {
        // consume everything up until a second '|'
        if c == '|' {
            terminated = true;
            break;
        }

        // handle escaping of bar characters inside
        // of extensions such as '\|'.
        if c == '\\' && state.chars.peek() == Some(&'|') {
            content.push('|');
            state.chars.next();
        } else {
            content.push(c);
        }
    }

    // unterminated extensions are interpreted as
    // just normal text
    if !terminated {
        state.text_buffer.push_str(&format!("|{}", content));
        return;
    }

    state.push_buffer();

    let argv: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    state
        .result
        .push(Inline::Extension(argv[0].clone(), argv[1..].to_vec()));
}

/// Handle potential start and end tags
fn tag(first: char, state: &mut ParserState) {
    let mut consumed_chars = first.to_string();
    // ensure that the neighboring char is the same
    if !state.next_is(first) {
        state.text_buffer.push(first);
        return;
    }

    // we can now safely consume
    // the second char of the tag
    consumed_chars.push(state.chars.next().unwrap());

    // (should never cause an exception)
    let tag = char_to_tag(first).expect("The char is not representing a tag");

    // Check if it is an closing tag
    if state.is_closing_tag(&tag) {
        if state.prev_is_whitespace {
            // closing tags can't come directly after a whitespace
            // so we abort and add the consumed chars to the buffer
            state.text_buffer.push_str(&consumed_chars);
            return;
        }
    
        // seems like we got a valid closing tag
        state.open_tags.pop();
        state.push_buffer();
        state.result.push(Inline::End(tag));
        return;
    }
    
    // otherwise... it's an opening tag
    if let Some(next) = state.chars.peek() {
        if next.is_whitespace() {
            // not a valid opening tag if it starts with whitespace
            // so we will abort and just push
            // the chars consumed so far to the buffer
            state.text_buffer.push_str(&consumed_chars);
            return;
        }
    }
    state.open_tags.push(tag.clone());
    state.push_buffer();
    state.result.push(Inline::Begin(tag));
}

/// Given a char, return the styling tag it
/// represents
fn char_to_tag(c: char) -> Option<Tag> {
    match c {
        '*' => Some(Tag::Bold),
        '/' => Some(Tag::Italic),
        '_' => Some(Tag::Subscript),
        '^' => Some(Tag::Superscript),
        '=' => Some(Tag::Underline),
        '~' => Some(Tag::Strikethrough),
        _ => None,
    }
}

fn tag_to_string(tag: &Tag) -> String {
    match tag {
        &Tag::Bold => "**",
        &Tag::Italic => "//",
        &Tag::Strikethrough => "~~",
        &Tag::Subscript => "__",
        &Tag::Superscript => "^^",
        &Tag::Underline => "==",
    }
    .to_string()
}

/// Handle potential escape characters and add them to the result vec
fn escape(state: &mut ParserState) {
    lazy_static! {
        // a prefix tree maping all the possible special escape characters
        static ref ESCAPE_TRIE: TSTMap<EscapeChar> = tstmap! {
            "alpha" => EscapeChar::Alpha,
            "beta" => EscapeChar::Beta,
            "gamma" => EscapeChar::GammaLower,
            "Gamma" => EscapeChar::GammaUpper,
            "delta" => EscapeChar::DeltaLower,
            "Delta" => EscapeChar::DeltaUpper,
            "epsilon" => EscapeChar::Epsilon,
            "varepsilon" => EscapeChar::EpsilonVar,
            "zeta" => EscapeChar::Zeta,
            "eta" => EscapeChar::Eta,
            "theta" => EscapeChar::ThetaLower,
            "Theta" => EscapeChar::ThetaUpper,
            "vartheta" => EscapeChar::ThetaVar,
            "iota" => EscapeChar::Iota,
            "kappa" => EscapeChar::Kappa,
            "lambda" => EscapeChar::LambdaLower,
            "Lambda" => EscapeChar::LambdaUpper,
            "mu" => EscapeChar::Mu,
            "nu" => EscapeChar::Nu,
            "xi" => EscapeChar::XiLower,
            "Xi" => EscapeChar::XiUpper,
            "pi" => EscapeChar::PiLower,
            "Pi" => EscapeChar::PiUpper,
            "rho" => EscapeChar::Rho,
            "varrho" => EscapeChar::RhoVar,
            "sigma" => EscapeChar::SigmaLower,
            "Sigma" => EscapeChar::SigmaUpper,
            "tau" => EscapeChar::Tau,
            "upsilon" => EscapeChar::UpsilonLower,
            "Upsilon" => EscapeChar::UpsilonUpper,
            "phi" => EscapeChar::PhiLower,
            "Phi" => EscapeChar::PhiUpper,
            "varphi" => EscapeChar::PhiVar,
            "chi" => EscapeChar::Chi,
            "psi" => EscapeChar::PsiLower,
            "Psi" => EscapeChar::PsiUpper,
            "omega" => EscapeChar::OmegaLower,
            "Omega" => EscapeChar::OmegaUpper,

            "endash" => EscapeChar::EnDash,
            "emdash" => EscapeChar::EmDash,

            "right" => EscapeChar::RightThin,
            "Right" => EscapeChar::RightBold,
            "left" => EscapeChar::LeftThin,
            "Left" => EscapeChar::LeftBold,
            "up" => EscapeChar::UpThin,
            "Up" => EscapeChar::UpBold,
            "down" => EscapeChar::DownThin,
            "Down" => EscapeChar::DownBold,
            // escaping lambda
            "*" => EscapeChar::Asterisk,
            "^" => EscapeChar::Caret,
            "_" => EscapeChar::Underscore,
            "/" => EscapeChar::ForwardSlash,
            "\\" => EscapeChar::BackSlash,
            "=" => EscapeChar::Equal,
            "~" => EscapeChar::Tilde,
            "|" => EscapeChar::Bar,
            "tableflip" => EscapeChar::TableFlip,
        };
    }

    // take all chars up until the next whitespace
    let word: String = state
        .chars
        .clone()
        .take_while(|c| !c.is_whitespace())
        .collect();

    let escape_char = ESCAPE_TRIE.longest_prefix(&word);

    if escape_char.is_empty() {
        // there were no matches, just add the backslash
        // and continue parsing the rest of the paragraph
        state.text_buffer.push('\\');
        return;
    }

    // we seem to have found a match,
    // first we advance the iterator by the correct amount
    // TODO: repace with advance_by once stable.
    for _ in 0..escape_char.len() {
        state.chars.next();
    }

    state.push_buffer();

    state.result.push(Inline::Escaped(
        ESCAPE_TRIE
            .get(escape_char)
            .expect("Failed to find escape char (should not be to happen)")
            .to_owned(),
    ));
}
