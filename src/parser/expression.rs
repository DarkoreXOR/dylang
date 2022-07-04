use super::ast_visitor::AstVisitor;

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Integer(i64),
}

impl Expression {
    #[allow(unused)]
    pub fn accept<R, E, V>(
        &mut self,
        mut visitor: impl AstVisitor<Value = R, Error = E>
    ) -> Result<R, E> {
        visitor.visit_expression(self)
    }
}
