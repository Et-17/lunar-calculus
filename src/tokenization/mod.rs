mod token;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub use token::*;

pub fn tokenize_file(file: &mut BufReader<File>) -> Result<Vec<Vec<Token>>, Vec<String>> {
    file.lines()
        .map(|line| tokenize_line(line.unwrap().as_str()))
        .collect()
}

pub fn tokenize_line(line: &str) -> Result<Vec<Token>, Vec<String>> {
    let mut start: usize;
    let mut current: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let line_chars: Vec<char> = line.chars().collect();
    while current < line_chars.len() {
        match line_chars[current] {
            '(' => {
                tokens.push(Token::new(TokenTypes::OpeningParenthesis, "(".to_string()));
                current += 1;
            }
            ')' => {
                tokens.push(Token::new(TokenTypes::ClosingParenthesis, ")".to_string()));
                current += 1
            }
            'l' => {
                if line.len() - current < 4 {
                    errors.push("Unrecognized token".to_string());
                    current += 1;
                    continue;
                }
                if &line[current..current + 3] == "let" && line_chars[current + 3].is_whitespace() {
                    tokens.push(Token::new(TokenTypes::Let, "let".to_string()));
                    current += 3;
                    continue;
                }
                start = current;
                while line.len() - current > 1 && is_identifier_char(line_chars[current]) {
                    current += 1;
                }
                tokens.push(Token::new(
                    TokenTypes::IdentifierName,
                    line[start..current + 1].to_string(),
                ));
            }
            '=' => {
                if line_chars.len() - current < 2 {
                    errors.push("Unrecognized token".to_string());
                    current += 1;
                    continue;
                }
                if line_chars[current + 1] == '>' {
                    tokens.push(Token::new(TokenTypes::DefinitionArrow, "=>".to_string()));
                    current += 2;
                    continue;
                }
                errors.push("Unrecognized token".to_string());
                current += 1;
            }
            '\\' => {
                tokens.push(Token::new(TokenTypes::LambdaSlash, "\\".to_string()));
                current += 1;
                continue;
            }
            ';' => {
                tokens.push(Token::new(
                    TokenTypes::LambdaArgumentSeparator,
                    ";".to_string(),
                ));
                current += 1;
                continue;
            }
            '-' => {
                if line_chars.len() - current < 2 {
                    errors.push("Unrecognized token".to_string());
                    current += 1;
                    continue;
                }
                if line_chars[current + 1] == '>' {
                    tokens.push(Token::new(TokenTypes::LambdaArrow, "->".to_string()));
                    current += 2;
                    continue;
                }
                errors.push("Unrecognized token".to_string());
                current += 1;
            }
            c => {
                if c.is_whitespace() {
                    current += 1;
                    continue;
                }
                if !is_identifier_char(c) {
                    errors.push("Unrecognized token".to_string());
                    current += 1;
                    continue;
                }
                start = current;
                while line.len() - current >= 1 && is_identifier_char(line_chars[current]) {
                    current += 1;
                }
                tokens.push(Token::new(
                    TokenTypes::IdentifierName,
                    line[start..current].to_string(),
                ));
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    } else {
        return Ok(tokens);
    }
}

fn is_identifier_char(chr: char) -> bool {
    chr.is_alphanumeric()
}
