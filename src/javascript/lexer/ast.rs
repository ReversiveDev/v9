use super::{Token, Tokenizer};

pub struct Ast {
    tokenizer: Tokenizer,
}

impl Ast {
    pub fn new(code: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(code),
        }
    }

    fn eof(&self) -> bool {
        match self.tokenizer.peek() {
            Token::Eof => true,
            _ => false,
        }
    }

    pub fn next(&mut self) -> Option<Node> {
        if !self.eof() {
            Some(Node::Expression(self.build_expression()))
        } else {
            None
        }
    }

    fn build_expression(&mut self) -> Expression {
        self.build_addiction()
    }

    fn build_addiction(&mut self) -> Expression {
        let mut expr = self.build_multiplication();

        loop {
            match self.tokenizer.peek() {
                Token::Plus | Token::Minus => {
                    let op = self.tokenizer.next();
                    expr = Expression::BinaryExpression(
                        Box::new(expr),
                        Box::new(self.build_multiplication()),
                        op.clone(),
                    );
                }
                _ => return expr,
            }
        }
    }

    fn build_multiplication(&mut self) -> Expression {
        let mut expr = self.build_primary();

        loop {
            match self.tokenizer.peek() {
                Token::Multiply | Token::Divide => {
                    let op = self.tokenizer.next();
                    expr = Expression::BinaryExpression(
                        Box::new(expr),
                        Box::new(self.build_primary()),
                        op.clone(),
                    );
                }
                _ => return expr,
            }
        }
    }

    fn build_primary(&mut self) -> Expression {
        let token = self.tokenizer.next();

        match token {
            Token::Number(value) => Expression::NumberLiteral(value),
            token => unreachable!("Unknown token {:?}", token),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryExpression(Box<Expression>, Box<Expression>, Token),
    StringLiteral(String),
    NumberLiteral(f32),
}

#[derive(Debug, Clone)]
pub enum Statement {}
