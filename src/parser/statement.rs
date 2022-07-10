use super::expression::Expression;
use super::ast_visitor::AstVisitor;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Let {
        name: String,
        expression: Option<Expression>,
    },
    //Return(Box<Expression>),
    //Function(Symbol, Vec<Statement>),
}

impl Statement {
    #[allow(unused)]
    pub fn accept<V, E>(
        &mut self,
        mut visitor: impl AstVisitor<Value = V, Error = E>
    ) -> Result<V, E> {
        visitor.visit_statement(self)
    }
}
