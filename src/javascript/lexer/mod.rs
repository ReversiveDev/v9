use lexical::parse_partial;
use std::str;

pub mod ast;

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut offset = 0;
    let input_bytes = code.as_bytes();

    // [Otimização] Obtém o tamanho total do código fora
    // do loop para evitar chamadas desnecessárias
    let code_len = code.len();

    // [Otimização] Define uma capacidade inicial baseada no tamanho do código
    // para evitar alocações dentro do loop
    let mut tokens: Vec<Token> = Vec::with_capacity(code_len / 2);

    while offset < code_len {
        let token: Token = match input_bytes[offset] {
            b'0'..=b'9' => {
                let (value, consumed) = parse_partial(&input_bytes[offset..]).unwrap();
                if consumed > 1 {
                    offset += consumed - 1;
                }
                Token::Number(value)
            }
            b'a'..=b'z' | b'A'..=b'Z' => {
                let start = offset;
                while offset < code_len && input_bytes[offset].is_ascii_alphabetic() {
                    offset += 1;
                }

                let chars = &input_bytes[start..offset];

                tokens.push(Token::Identifier(
                    str::from_utf8(chars).unwrap().to_string(),
                ));

                continue;
            }
            b'"' | b'\'' => {
                let quote = input_bytes[offset];
                offset += 1;

                let start = offset;
                while offset < code_len
                    && input_bytes[offset].is_ascii_alphabetic()
                    && input_bytes[offset] != quote
                {
                    offset += 1;
                }

                let chars = &input_bytes[start..offset];

                // Skip the closing quote
                if input_bytes[offset] == quote {
                    offset += 1;
                }

                tokens.push(Token::String(
                    str::from_utf8(chars).unwrap().to_string(),
                ));

                continue;
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
            _ => {
                return Err(format!(
                    "Unknown token: '{}'",
                    str::from_utf8(&input_bytes[offset..offset + 1])
                        .unwrap()
                        .to_string()
                ))
            }
        };

        tokens.push(token);
        offset += 1;
    }

    tokens.push(Token::Eof);
    Ok(tokens)
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
