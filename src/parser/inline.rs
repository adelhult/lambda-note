use lazy_static::lazy_static;
use tst::{tstmap, TSTMap};

use super::{EscapeChar, Inline, Tag};
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
/// all the inline elements found inside of it.
pub fn parse_inline<'a>(source: &'a str) -> Vec<Inline> {
    let mut state = ParserState::new(source);

    while let Some(current) = state.chars.next() {
        match current {
            '\n' => state.text_buffer.push(' '), // kanske inte helt rätt? Borde peeka och kolla om det finns flera chars
            '\\' => escape(&mut state),
            '|' => extension(&mut state),
            '*' | '/' | '=' | '_' | '^' | '~' => tag(&mut state),
            _ => state.text_buffer.push(current),
        }
    }

    // finally, push the last bit of the buffer
    state.push_buffer();

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
fn tag(state: &mut ParserState) {
    // TODO
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
