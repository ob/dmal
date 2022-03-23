mod cursor;

#[cfg(test)]
mod tests;

use self::TokenKind::*;
use cursor::Cursor;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    LineComment,
    Ident,
    InvalidIdent,
    NewLine,
    Whitespace,
    /// "."
    Dot,
    Unknown,
}

pub fn is_newline(c: char) -> bool {
    matches!(
        c,
        '\u{000D}' // \r
        | '\u{000A}' // \n
        // NEXT LINE from latin1
        | '\u{0085}'
    )
}

/// True if `c` is considered whitespace
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{0020}' // space

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub fn is_id_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

pub fn is_id_continue(c: char) -> bool {
    c.is_alphanumeric()
}

/// The passed string is lexically an identifier.
pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(start) = chars.next() {
        is_id_start(start) && chars.all(is_id_continue)
    } else {
        false
    }
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.advance_token())
        }
    })
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    fn advance_token(&mut self) -> Token {
        let first_char = self.eat().unwrap();
        let token_kind = match first_char {
            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),
            c if is_id_start(c) => self.ident(),

            // One-symbol tokens.
            '.' => Dot,

            _ => Unknown,
        };
        Token::new(token_kind, self.len_consumed())
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        // Start is already eaten, eat the rest of identifier.
        self.eat_while(is_id_continue);
        // Known prefixes must have been handled earlier. So if
        // we see a prefix here, it is definitely an unknown prefix.
        match self.first() {
            c if !c.is_ascii() && unic_emoji_char::is_emoji(c) => InvalidIdent,
            _ => Ident,
        }
    }
}
