// This software is available under the terms of the MIT license.

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub line: u32,
    pub col: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Ident,
    Const,
    Fn,
    Mut,

    Arrow,
    Asterisk,
    Comma,
    LParenthesis,
    Minus,
    RParenthesis,
    Semicolon,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub src: &'a [u8],
    pub loc: Location,
}

pub struct Lexer<'a> {
    name: &'a str,
    src: &'a [u8],
    loc: Location,
}

impl<'a> Lexer<'a> {
    pub fn name (&self) -> &'a str { self.name }

    pub fn new (name: &'a str, src: &'a [u8]) -> Lexer<'a> {
        Lexer {
            name, src,
            loc: Location { line: 1, col: 1 },
        }
    }
}

impl<'a> Lexer<'a> {
    fn consume (&mut self) {
        if self.src.is_empty() {
            return;
        }

        match self.src[0] {
            b'\n' => {
                self.src = &self.src[1..];
                self.loc.line += 1;
                self.loc.col = 1;
            },

            b'\r' => {
                if self.src.len() >= 2 && self.src[1] == b'\n' {
                    self.src = &self.src[2..];
                } else {
                    self.src = &self.src[1..];
                }

                self.loc.line += 1;
                self.loc.col = 1;
            },

            _ => {
                self.src = &self.src[1..];
                self.loc.col += 1;
            }
        }
    }

    fn consume_n (&mut self, mut n: usize) {
        while n > 0 {
            self.consume();
            n -= 1;
        }
    }

    fn consume_whitespace (&mut self) {
        loop {
            if self.src.is_empty() {
                return;
            }

            match self.src[0] {
                b' ' | b'\n' | b'\r' => self.consume(),
                _ => break
            }
        }
    }

    fn emit (&mut self, kind: TokenKind, len: usize) -> Option<Token<'a>> {
        let token = Token {
            kind,
            src: &self.src[..len],
            loc: self.loc,
        };

        self.consume_n(len);
        Some(token)
    }

    fn emit_ident (&mut self) -> Option<Token<'a>> {
        let mut len = 0;

        loop {
            if len == self.src.len() {
                break;
            }

            match self.src[len] {
                b'_' | b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => len += 1,
                _ => break
            }
        }

        let token = Token {
            kind: match &self.src[..len] {
                b"const" => TokenKind::Const,
                b"fn" => TokenKind::Fn,
                b"mut" => TokenKind::Mut,
                _ => TokenKind::Ident
            },
            src: &self.src[..len],
            loc: self.loc,
        };

        self.consume_n(len);
        Some(token)
    }

    fn emit_match (&mut self, kind: TokenKind, seq: &str) -> Option<Token<'a>> {
        let seq = seq.as_bytes();
        let len = seq.len();

        if self.src.len() >= len && &self.src[..len] == seq {
            self.emit(kind, len)
        } else {
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next (&mut self) -> Option<Token<'a>> {
        self.consume_whitespace();

        if self.src.is_empty() {
            return None;
        }

        match self.src[0] {
            b'_' | b'A'...b'Z' | b'a'...b'z' => self.emit_ident(),

            b'(' => self.emit(TokenKind::LParenthesis, 1),
            b')' => self.emit(TokenKind::RParenthesis, 1),
            b'*' => self.emit(TokenKind::Asterisk, 1),
            b',' => self.emit(TokenKind::Comma, 1),
            b'-' => self.emit_match(TokenKind::Arrow, "->")
                    .or_else(|| self.emit(TokenKind::Minus, 1)),
            b';' => self.emit(TokenKind::Semicolon, 1),

            0x20...0x7e => panic!("{}: unexpected symbol '{}' at {}~{}",
                                  self.name,
                                  self.src[0] as char,
                                  self.loc.line,
                                  self.loc.col),

            _ => panic!("{}: invalid byte(s) at {}~{}", self.name, self.loc.line, self.loc.col)
        }
    }
}
