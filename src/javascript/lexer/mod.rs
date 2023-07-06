use lexical::parse_partial;
use std::str;

pub mod ast;

pub struct Tokenizer {
    offset: usize,
    code: Vec<u8>,
    code_len: usize,
}

impl Tokenizer {
    pub fn new(code: &str) -> Self {
        let code = code.as_bytes().to_vec();
        Self {
            offset: 0,
            code_len: code.len(),
            code,
        }
    }

    fn eof(&self) -> bool {
        self.offset >= self.code_len
    }

    pub fn peek(&self) -> Token {
        if self.eof() {
            Token::Eof
        } else {
            let mut offset = self.offset;
            let token;
            loop {
                token = match self.code[offset] {
                    b'0'..=b'9' => {
                        let (value, _) = parse_partial(&self.code[offset..]).unwrap();
                        Token::Number(value)
                    }
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        let start = offset;
                        while !self.eof() && self.code[offset].is_ascii_alphabetic() {
                            offset += 1;
                        }

                        let chars = &self.code[start..offset];

                        return Token::Identifier(str::from_utf8(chars).unwrap().to_string());
                    }
                    b'"' | b'\'' => {
                        let quote = self.code[offset];
                        offset += 1;

                        let start = offset;
                        while !self.eof()
                            && self.code[offset].is_ascii_alphabetic()
                            && self.code[offset] != quote
                        {
                            offset += 1;
                        }

                        let chars = &self.code[start..offset];

                        return Token::String(str::from_utf8(chars).unwrap().to_string());
                    }
                    b'+' => Token::Plus,
                    b'-' => Token::Minus,
                    b'*' => Token::Multiply,
                    b'/' => Token::Divide,
                    b'=' => Token::Assignment,
                    b';' => Token::Semicolon,
                    b':' => Token::Colon,
                    b',' => Token::Comma,
                    b'.' => Token::Dot,
                    b'(' => Token::LParen,
                    b')' => Token::RParen,
                    b'{' => Token::LBrace,
                    b'}' => Token::RBrace,
                    b'[' => Token::LBracket,
                    b']' => Token::RBracket,
                    b' ' | b'\t' | b'\n' | b'\r' => {
                        offset += 1;
                        continue;
                    }
                    _ => unreachable!(
                        "Unknown token '{}'",
                        str::from_utf8(&self.code[offset..offset + 1])
                            .unwrap()
                            .to_string()
                    ),
                };

                break;
            }

            token
        }
    }

    pub fn next(&mut self) -> Token {
        if self.eof() {
            Token::Eof
        } else {
            let token;
            loop {
                token = match self.code[self.offset] {
                    b'0'..=b'9' => {
                        let (value, consumed) = parse_partial(&self.code[self.offset..]).unwrap();
                        if consumed > 1 {
                            self.offset += consumed - 1;
                        }
                        Token::Number(value)
                    }
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        let start = self.offset;
                        while !self.eof() && self.code[self.offset].is_ascii_alphabetic() {
                            self.offset += 1;
                        }

                        let chars = &self.code[start..self.offset];

                        return Token::Identifier(str::from_utf8(chars).unwrap().to_string());
                    }
                    b'"' | b'\'' => {
                        let quote = self.code[self.offset];
                        self.offset += 1;

                        let start = self.offset;
                        while !self.eof()
                            && self.code[self.offset].is_ascii_alphabetic()
                            && self.code[self.offset] != quote
                        {
                            self.offset += 1;
                        }

                        let chars = &self.code[start..self.offset];

                        // Skip the closing quote
                        if self.code[self.offset] == quote {
                            self.offset += 1;
                        }

                        return Token::String(str::from_utf8(chars).unwrap().to_string());
                    }
                    b'+' => Token::Plus,
                    b'-' => Token::Minus,
                    b'*' => Token::Multiply,
                    b'/' => Token::Divide,
                    b'=' => Token::Assignment,
                    b';' => Token::Semicolon,
                    b':' => Token::Colon,
                    b',' => Token::Comma,
                    b'.' => Token::Dot,
                    b'(' => Token::LParen,
                    b')' => Token::RParen,
                    b'{' => Token::LBrace,
                    b'}' => Token::RBrace,
                    b'[' => Token::LBracket,
                    b']' => Token::RBracket,
                    b' ' | b'\t' | b'\n' | b'\r' => {
                        self.offset += 1;
                        continue;
                    }
                    _ => unreachable!(
                        "Unknown token '{}'",
                        str::from_utf8(&self.code[self.offset..self.offset + 1])
                            .unwrap()
                            .to_string()
                    ),
                };

                break;
            }

            self.offset += 1;
            token
        }
    }
}

#[derive(Clone, Debug)]
pub enum Token {
    // Values
    Number(f32),
    Identifier(String),
    String(String),

    // Symbols
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Assignment,
    Semicolon,
    Comma,
    Colon,
    Dot,

    // Operadores matemáticos
    Plus,
    Minus,
    Multiply,
    Divide,

    Eof,
}
