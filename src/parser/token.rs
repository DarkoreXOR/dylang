#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String), // Keyword or Identifier
    NumberLiteral(i64),
    StringLiteral(String),
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    Equal, // =
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Comma, // ,
    Colon, // :
    Semicolon, // ;
}
