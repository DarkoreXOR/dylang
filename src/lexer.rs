use std::borrow::Borrow;

use crate::token::Token;

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

    pub fn tokenize(&mut self) -> &Vec<Token> {
        self.source_offset = 0;
        self.tokens.clear();

        while let Some(current_character) = self.peek() {
            match current_character {
                ch if ch.is_whitespace() => self.next(),

                ch if ch.is_alphabetic() || ch == '_' => {
                    let token = self.tokenize_identifier();
                    self.add_token(token);
                }

                ch if ch == '"' => {
                    let token = self.tokenize_string_literal();
                    self.add_token(token);
                }

                ch if ch.is_digit(10) => {
                    let token = self.tokenize_number_literal();
                    self.add_token(token);
                }

                '+' | '-' | '=' | '(' | ')' | '{' | '}' | ',' | ':' | ';' => {
                    let token = self.tokenize_single_character();
                    self.add_token(token);
                }

                ch => {
                    println!("skip unknown character: `{}`", ch);
                    self.next();
                }
            }
        }

        self.tokens.push(Token::EOF);
        self.tokens.borrow()
    }

    fn tokenize_identifier(&mut self) -> Token {
        let mut identifier_builder = String::new();
        let mut first = true;
        
        while let Some(current_char) = self.peek() {
            let is_first_char_valid = current_char.is_alphabetic() ||
                current_char == '_';
            
            let is_tail_char_valid = is_first_char_valid ||
                current_char.is_digit(10);

            if first {
                first = false;

                if !is_first_char_valid {
                    break;
                }
            } else {
                if !is_tail_char_valid {
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

    fn tokenize_single_character(&mut self) -> Token {
        match self.peek_and_next().unwrap() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '=' => Token::Equal,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,

            //
            _ => unreachable!(),
        }
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
}
