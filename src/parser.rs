use crate::token::Token;

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum Expression {
    BinaryOp(BinaryOperator, Box<Expression>, Box<Expression>),
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
    tokens: &'a Vec<Token>,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
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
        let mut expr = self
            .parse_primary()
            .unwrap();

        loop {
            if let Token::Plus = self.peek() {
                self.next();

                let right_expr = self
                    .parse_primary()
                    .unwrap();

                expr = Box::new(
                    Expression::BinaryOp(
                        BinaryOperator::Plus,
                        expr,
                        right_expr
                    )
                );

                continue;
            }

            if let Token::Minus = self.peek() {
                self.next();

                let right_expr = self
                    .parse_primary()
                    .unwrap();

                expr = Box::new(
                    Expression::BinaryOp(
                        BinaryOperator::Minus,
                        expr,
                        right_expr
                    )
                );

                continue;
            }

            break;
        }
        
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Box<Expression>, ()> {
        let token = self.peek();

        if token != Token::EOF {
            self.next();

            let expression = match token {
                Token::NumberLiteral(n) => Expression::Integer(n),

                //
                _ => unreachable!(),
            };

            return Ok(Box::new(expression));
        }

        Err(())
    }

    fn peek(&self) -> Token {
        if self.offset < self.tokens.len() {
            self.tokens[self.offset].clone()
        } else {
            Token::EOF
        }
    }

    fn next(&mut self) {
        self.offset += 1;
    }
}
