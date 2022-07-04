use super::token::Token;
use super::statement::Statement;
use super::expression::{Expression, BinaryOperator, UnaryOperator};

// #[derive(Debug)]
// pub struct Symbol {

// }

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

    pub fn parse(&mut self) -> Result<Statement, ()> {
        self.offset = 0;

        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Result<Statement, ()> {
        let expression = self.parse_expression()?;

        Ok(Statement::Expression(*expression))
    }

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
        let unary_operator_opt = match self.peek() {
            Some(Token::Minus) => Some(UnaryOperator::Minus),
            Some(Token::Plus) => Some(UnaryOperator::Plus),
            _ => None,
        };

        if let Some(unary_operator) = unary_operator_opt {
            self.next();
            let expression = self.parse_unary()?;
            Ok(Box::new(Expression::Unary(unary_operator, expression)))
        } else {
            self.parse_primary()
        }
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

#[cfg(test)]
mod tests {
    use super::super::expression::BinaryOperator;
    use super::super::parser::UnaryOperator;
    use super::super::token::Token;
    use super::super::expression::Expression;
    use super::Parser;

    #[test]
    fn parse_primary_number_0() {
        let tokens = [
            Token::NumberLiteral(0)
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Integer(0)
            )
        );

        assert_eq!(parser.parse_primary(), result);
    }

    #[test]
    fn parse_primary_number_12345() {
        let tokens = [
            Token::NumberLiteral(12345)
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Integer(12345)
            )
        );

        assert_eq!(parser.parse_primary(), result);
    }

    #[test]
    fn parse_unary_negative() {
        let tokens = [
            Token::Minus,
            Token::NumberLiteral(1)
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Unary(
                    UnaryOperator::Minus,
                    Box::new(
                        Expression::Integer(1)
                    )
                )
            )
        );

        assert_eq!(parser.parse_unary(), result);
    }

    #[test]
    fn parse_unary_positive() {
        let tokens = [
            Token::Plus,
            Token::NumberLiteral(999)
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Unary(
                    UnaryOperator::Plus,
                    Box::new(
                        Expression::Integer(999)
                    )
                )
            )
        );

        assert_eq!(parser.parse_unary(), result);
    }

    #[test]
    fn parse_binary_number_plus_number() {
        let tokens = [
            Token::NumberLiteral(1),
            Token::Plus,
            Token::NumberLiteral(2),
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Binary(
                    BinaryOperator::Plus,
                    Box::new(
                        Expression::Integer(1)
                    ),
                    Box::new(
                        Expression::Integer(2)
                    ),
                )
            )
        );

        assert_eq!(parser.parse_addition(), result);
    }

    #[test]
    fn parse_binary_number_minus_number() {
        let tokens = [
            Token::NumberLiteral(5),
            Token::Minus,
            Token::NumberLiteral(3),
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Binary(
                    BinaryOperator::Minus,
                    Box::new(
                        Expression::Integer(5)
                    ),
                    Box::new(
                        Expression::Integer(3)
                    ),
                )
            )
        );

        assert_eq!(parser.parse_addition(), result);
    }

    #[test]
    fn parse_binary_number_multiply_number() {
        let tokens = [
            Token::NumberLiteral(9),
            Token::Star,
            Token::NumberLiteral(6),
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Binary(
                    BinaryOperator::Multiply,
                    Box::new(
                        Expression::Integer(9)
                    ),
                    Box::new(
                        Expression::Integer(6)
                    ),
                )
            )
        );

        assert_eq!(parser.parse_addition(), result);
    }

    #[test]
    fn parse_binary_number_divide_number() {
        let tokens = [
            Token::NumberLiteral(4),
            Token::Slash,
            Token::NumberLiteral(2),
        ];

        let mut parser = Parser::new(&tokens);

        let result = Ok(
            Box::new(
                Expression::Binary(
                    BinaryOperator::Divide,
                    Box::new(
                        Expression::Integer(4)
                    ),
                    Box::new(
                        Expression::Integer(2)
                    ),
                )
            )
        );

        assert_eq!(parser.parse_addition(), result);
    }
}

