use std::collections::HashMap;
use super::token::Token;

pub struct Lexer {
    source_chars: Vec<char>,
    source_offset: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source_chars: source.chars().collect(),
            source_offset: 0,
            tokens: Vec::new(),
        }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn tokenize(&mut self) -> &[Token] {
        self.source_offset = 0;
        self.tokens.clear();

        while let Some(current_character) = self.peek() {
            let token_opt = match current_character {
                ch if ch.is_whitespace() => {
                    self.next();
                    None
                }

                ch if ch.is_alphabetic() || ch == '_' => {
                    Some(self.tokenize_identifier())
                }

                ch if ch == '"' => {
                    Some(self.tokenize_string_literal())
                }

                ch if ch.is_digit(10) => {
                    Some(self.tokenize_number_literal())
                }

                ch => match Self::single_character_to_token(ch) {
                    Some(token) => {
                        self.next();
                        Some(token)
                    }
                    None => {
                        println!("skip unknown character: `{}`", ch);
                        self.next();
                        None
                    }
                }
            };

            if let Some(token) = token_opt {
                self.add_token(token);
            }
        }

        &self.tokens
    }

    fn tokenize_identifier(&mut self) -> Token {
        let mut identifier_builder = String::new();
        let mut first_iteration = true;
        
        while let Some(current_char) = self.peek() {
            let valid_first_char = current_char.is_alphabetic() || current_char == '_';
            let valid_tail_char = valid_first_char || current_char.is_digit(10);

            if first_iteration {
                first_iteration = false;

                if !valid_first_char {
                    break;
                }
            } else {
                if !valid_tail_char {
                    break;
                }
            }

            identifier_builder.push(current_char);

            self.next();
        }

        Token::Identifier(identifier_builder)
    }

    fn tokenize_string_literal(&mut self) -> Token {
        let mut string_builder = String::new();

        self.take_if(|ch| ch == '"')
            .expect("string literal begin quote");

        while let Some(current_char) = self.take_if(|ch| ch != '"') {
            string_builder.push(current_char);
        }

        self.take_if(|ch| ch == '"')
            .expect("string literal end quote");

        Token::StringLiteral(string_builder)
    }

    fn tokenize_number_literal(&mut self) -> Token {
        let mut digits_builder = String::new();

        while let Some(current_char) = self.take_if(|ch| ch.is_digit(10)) {
            digits_builder.push(current_char);
        }

        let parsed_number = digits_builder
            .parse()
            .unwrap();

        Token::NumberLiteral(parsed_number)
    }

    fn next(&mut self) {
        self.source_offset += 1;
    }

    fn peek(&self) -> Option<char> {
        if self.source_offset < self.source_chars.len() {
            Some(self.source_chars[self.source_offset])
        } else {
            None
        }
    }

    #[allow(unused)]
    fn peek_and_next(&mut self) -> Option<char> {
        let result = self.peek();
        self.next();
        result
    }

    fn take_if<F>(&mut self, f: F) -> Option<char> where F: Fn(char) -> bool {
        let current = self.peek();

        if let Some(current_char) = current {
            if f(current_char) {
                self.next();
                return Some(current_char);
            }
        }
        
        None
    }

    const fn single_character_to_token(character: char) -> Option<Token> {
        match character {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '=' => Some(Token::Equal),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            ';' => Some(Token::Semicolon),
            _ => None,
        }
    }
}
