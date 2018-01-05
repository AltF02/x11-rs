// This software is available under the terms of the MIT license.

use lex;

#[derive(Debug)]
pub enum Type<'a> {
    Ident(&'a [u8]),
    ConstPtr(Box<Type<'a>>),
    MutPtr(Box<Type<'a>>),
}

#[derive(Debug)]
pub struct Fn<'a> {
    pub name: &'a [u8],
    pub return_ty: Option<Type<'a>>,
    pub params: Vec<Type<'a>>,
}

pub struct Parser<'a> {
    lex: lex::Lexer<'a>,
    next: Option<lex::Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new (lex: lex::Lexer<'a>) -> Parser<'a> {
        Parser {
            lex,
            next: None,
        }
    }
}

impl<'a> Parser<'a> {
    fn accept (&mut self, kind: lex::TokenKind) -> Option<&'a [u8]> {
        let t = self.token();

        if t.kind == kind {
            Some(t.src)
        } else {
            self.next = Some(t);
            None
        }
    }

    fn accept_ty (&mut self) -> Option<Type<'a>> {
        if let Some(id) = self.accept(lex::TokenKind::Ident) {
            Some(Type::Ident(id))
        } else if self.accept(lex::TokenKind::Asterisk).is_some() {
            if self.accept(lex::TokenKind::Const).is_some() {
                Some(Type::ConstPtr(Box::new(self.expect_ty())))
            } else if self.accept(lex::TokenKind::Mut).is_some() {
                Some(Type::MutPtr(Box::new(self.expect_ty())))
            } else {
                self.unexpect();
            }
        } else {
            None
        }
    }

    fn end (&mut self) -> bool {
        if self.next.is_some() {
            false
        } else {
            self.next = self.lex.next();
            self.next.is_none()
        }
    }

    fn expect (&mut self, kind: lex::TokenKind) -> &'a [u8] {
        let t = self.token();

        if t.kind != kind {
            panic!("{}: expected {:?} at {}~{}", self.lex.name(), kind, t.loc.line, t.loc.col);
        }

        t.src
    }

    fn expect_ty (&mut self) -> Type<'a> {
        match self.accept_ty() {
            None => {
                let t = self.next.take().unwrap();
                panic!("{}: expected type at {}~{}", self.lex.name(), t.loc.line, t.loc.col);
            }
            Some(t) => t,
        }
    }

    fn token (&mut self) -> lex::Token<'a> {
        match self.next.take() {
            None => match self.lex.next() {
                None => panic!("{}: unexpected end of file", self.lex.name()),
                Some(t) => t,
            },
            Some(t) => t,
        }
    }

    fn unexpect (&mut self) -> ! {
        let t = self.next.take().unwrap();
        panic!("{}: unexpected {:?} at {}~{}", self.lex.name(), t.kind, t.loc.line, t.loc.col);
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Fn<'a>;

    fn next (&mut self) -> Option<Fn<'a>> {
        if self.end() {
            return None;
        }

        let mut return_ty = None;
        let mut params = Vec::new();

        self.expect(lex::TokenKind::Fn);
        let name = self.expect(lex::TokenKind::Ident);
        self.expect(lex::TokenKind::LParenthesis);

        if self.accept(lex::TokenKind::RParenthesis).is_some() {
            // no params
        } else {
            params.push(self.expect_ty());

            loop {
                if self.accept(lex::TokenKind::Comma).is_some() {
                    params.push(self.expect_ty());
                } else {
                    self.expect(lex::TokenKind::RParenthesis);
                    break;
                }
            }
        }

        if self.accept(lex::TokenKind::Arrow).is_some() {
            return_ty = Some(self.expect_ty());
        }

        self.expect(lex::TokenKind::Semicolon);

        Some(Fn { name, return_ty, params })
    }
}
