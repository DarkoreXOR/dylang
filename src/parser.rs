use crate::token::Token;

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Expression {
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Integer(i64),
}

#[derive(Debug)]
pub struct Symbol {

}

/* #[derive(Debug)]
pub enum Statement {
    Return(Box<Expression>),
    Function(Symbol, Vec<Statement>),
} */

pub struct Parser<'a> {
    tokens: &'a [Token],
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            offset: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, ()> {
        self.offset = 0;

        self.parse_expression()
    }

/*     fn parse_statement() -> Result<Box<Statement>, ()> {
        Err(())
    } */

    fn parse_expression(&mut self) -> Result<Box<Expression>, ()> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Box<Expression>, ()> {
        let mut expression = self.parse_multiplication()?;

        loop {
            let binary_operator = match self.peek() {
                Some(Token::Plus) => BinaryOperator::Plus,
                Some(Token::Minus) => BinaryOperator::Minus,

                _ => break,
            };

            self.next();

            let right_expression = self.parse_multiplication()?;

            expression = Box::new(Expression::Binary(binary_operator, expression, right_expression));
        }
        
        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> Result<Box<Expression>, ()> {
        let mut expression = self.parse_unary()?;

        loop {
            let binary_operator = match self.peek() {
                Some(Token::Star) => BinaryOperator::Multiply,
                Some(Token::Slash) => BinaryOperator::Divide,

                _ => break,
            };

            self.next();

            let right_expression = self.parse_unary()?;

            expression = Box::new(Expression::Binary(binary_operator, expression, right_expression));
        }
        
        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Box<Expression>, ()> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Box<Expression>, ()> {
        if let Some(token) = self.peek() {
            let expression = match token {
                Token::NumberLiteral(n) => Expression::Integer(*n),

                //
                _ => unreachable!(),
            };
            
            self.next();

            return Ok(Box::new(expression));
        }

        Err(())
    }

    fn peek(&self) -> Option<&Token> {
        if self.offset < self.tokens.len() {
            Some(&self.tokens[self.offset])
        } else {
            None
        }
    }

    fn next(&mut self) {
        self.offset += 1;
    }
}
