use std::marker::PhantomData;

use super::{expression::{Expression, BinaryOperator, UnaryOperator}, statement::Statement};

pub trait AstVisitor {
    type Value;
    type Error;

    fn visit_statement(
        &mut self,
        statement: &Statement,
    ) -> Result<Self::Value, Self::Error>;

    fn visit_statement_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Self::Value, Self::Error>;

    fn visit_let_statement(
        &mut self,
        name: &str,
        expression: Option<&Expression>,
    ) -> Result<Self::Value, Self::Error>;

    fn visit_expression(
        &mut self,
        expression: &Expression
    ) -> Result<Self::Value, Self::Error>;

    fn visit_binary(
        &mut self,
        operator: &BinaryOperator,
        left_expression: &Expression,
        right_expression: &Expression
    ) -> Result<Self::Value, Self::Error>;

    fn visit_unary(
        &mut self,
        operator: &UnaryOperator,
        expression: &Expression
    ) -> Result<Self::Value, Self::Error>;

    fn visit_number(
        &mut self,
        value: i64
    ) -> Result<Self::Value, Self::Error>;
}

pub struct DefaultAstVisitor<V, E> { 
    _phantom: (PhantomData<V>, PhantomData<E>),
}

impl<V, E> DefaultAstVisitor<V, E> {
    pub fn new() -> Self {
        Self {
            _phantom: (PhantomData, PhantomData)
        }
    }
}

impl<V, E> AstVisitor for DefaultAstVisitor<V, E>
where
    V: Default
{
    type Value = V;
    type Error = E;

    fn visit_statement(
        &mut self,
        statement: &Statement,
    ) -> Result<V, E> {
        match statement {
            Statement::Expression(expression) => {
                self.visit_expression(expression)
            }

            Statement::Let { name, expression } => {
                self.visit_let_statement(name.as_str(), expression.as_ref())
            }
        }
    }

    fn visit_statement_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<V, E> {
        self.visit_expression(expression)?;
        Ok(Default::default())
    }

    fn visit_let_statement(
        &mut self,
        _name: &str,
        expression: Option<&Expression>,
    ) -> Result<V, E> {
        if let Some(expr) = expression {
            self.visit_expression(expr)?;
        }

        Ok(Default::default())
    }

    fn visit_expression(
        &mut self,
        expression: &Expression
    ) -> Result<V, E> {
        match expression {
            Expression::Binary(operator, left_expression, right_expression) =>
                self.visit_binary(operator, left_expression, right_expression),

            Expression::Unary(operator, expression) =>
                self.visit_unary(operator, expression),

            Expression::Integer(value) =>
                self.visit_number(*value),
        }
    }

    #[allow(unused)]
    fn visit_binary(
        &mut self,
        operator: &BinaryOperator,
        left_expression: &Expression,
        right_expression: &Expression
    ) -> Result<V, E> {
        self.visit_expression(left_expression)?;
        self.visit_expression(right_expression)?;

        Ok(Default::default())
    }

    #[allow(unused)]
    fn visit_unary(
        &mut self,
        operator: &UnaryOperator,
        expression: &Expression
    ) -> Result<V, E> {
        self.visit_expression(expression)?;

        Ok(Default::default())
    }

    #[allow(unused)]
    fn visit_number(&mut self, value: i64) -> Result<V, E> {
        Ok(Default::default())
    }
}
