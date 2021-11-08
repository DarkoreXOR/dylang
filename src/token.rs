#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String), // Keyword or Identifier
    NumberLiteral(i64),
    StringLiteral(String),
    Plus, // +
    Minus, // -
    Equal, // =
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Comma, // ,
    Colon, // :
    Semicolon, // ;
    EOF,
}
