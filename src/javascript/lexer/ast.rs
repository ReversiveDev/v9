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
            Some(self.build())
        } else {
            None
        }
    }

    fn build(&mut self) -> Node {
        self.build_variable_declaration()
    }

    fn build_variable_declaration(&mut self) -> Node {
        let expr: Expression = self.build_addiction();

        match expr.clone() {
            Expression::Identifier(keyword) => match keyword.as_str() {
                "var" | "let" | "const" => {
                    let mut declarations: Vec<Statement> = Vec::new();
                    while !self.eof() {
                        let identifier = self.build_primary();
                        match identifier {
                            Expression::Identifier(identifier) => {
                                let mut value: Option<Expression> = None;
                                match self.tokenizer.peek() {
                                    Token::Assignment => {
                                        self.tokenizer.next();
                                        value = Some(self.build_addiction());
                                    }
                                    Token::Semicolon => {
                                        self.tokenizer.next();
                                        break;
                                    }
                                    _ => panic!("Error"),
                                }

                                declarations.push(Statement::VariableDeclarator(
                                    identifier,
                                    Box::new(value),
                                ));
                            }
                            _ => panic!("Invalid variable declaration"),
                        }
                    }
                    Node::Statement(Statement::VariableDeclaration(declarations))
                }
                _ => Node::Expression(expr),
            },
            _ => Node::Expression(expr),
        }

        // let a = self.build_addiction();

        // Node::Statement(Statement::VariableDeclaration(vec![]))
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
            Token::Identifier(identifier) => Expression::Identifier(identifier),
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
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(Vec<Statement>),
    VariableDeclarator(String, Box<Option<Expression>>),
}
