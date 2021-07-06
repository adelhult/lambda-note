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
    // TODO: write a proper one to handle \| and other
    // edgecases
    let contents: Vec<String> = state
        .chars
        .by_ref()
        .take_while(|c| *c != '|')
        .collect::<String>()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    state.push_buffer();

    state.result.push(Inline::Extension(
        contents[0].clone(),
        contents[1..].to_vec(),
    ))
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

            "endash" => EscapeChar::EmDash,
            "emdash" => EscapeChar::EnDash,

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
