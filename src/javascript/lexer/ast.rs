use super::{tokenize, Token};

pub struct Ast {
    offset: usize,
    tokens: Vec<Token>,
    tokens_len: usize,
}

impl Ast {
    pub fn new(code: &str) -> Self {
        let tokens = tokenize(code).unwrap();
        Self {
            offset: 0,
            tokens_len: tokens.len(),
            tokens,
        }
    }

    fn peek(&self) -> &Token {
        if let Some(token) = self.tokens.get(self.offset) {
            token
        } else {
            unreachable!("Offset out of bounds");
        }
    }

    fn eat(&mut self) -> Token {
        let token = self.peek().clone();
        self.offset += 1;
        token
    }

    fn eof(&self) -> bool {
        self.offset >= self.tokens_len
            || match self.peek() {
                Token::Eof => true,
                _ => false,
            }
    }

    pub fn build(&mut self) -> Result<Statement, ()> {
        let mut nodes: Vec<Node> = Vec::new();

        while !self.eof() {
            nodes.push(Node::Expression(self.build_expression()));
        }

        Ok(Statement::Program(nodes))
    }

    fn build_expression(&mut self) -> Expression {
        self.build_addiction()
    }

    fn build_addiction(&mut self) -> Expression {
        let mut expr = self.build_multiplication();

        loop {
            match self.peek() {
                Token::Plus | Token::Minus => {
                    let op = self.eat();
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
            match self.peek() {
                Token::Multiply | Token::Divide => {
                    let op = self.eat();
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
        let token = self.eat();

        match token {
            Token::Number(value) => Expression::NumberLiteral(value),
            token => unreachable!("Unknown token {:?}", token),
        }
    }
}

#[derive(Debug, Clone)]
#[warn(dead_code)]
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
pub enum Statement {
    Program(Vec<Node>),
}
