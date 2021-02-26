use self::TokenKind::*;
use crate::cursor::Cursor;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Eof,

    Unknown,
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            cursor: Cursor::new(input),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.cursor.first() {
            c if is_whitespace(c) => Some(self.eat_whitespace()),
            _ => None,
        }
    }

    fn eat_whitespace(&mut self) -> Token {
        self.cursor.next();
        Token::new(Whitespace, self.cursor.len_consumed())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.cursor.is_eof() {
            return None;
        }

        self.next_token()
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    Lexer::new(input).collect()
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t')
}
