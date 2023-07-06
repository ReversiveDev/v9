use std::rc::Rc;

use self::values::Value;

use super::lexer::{
    ast::{Ast, Expression, Node, Statement},
    Token,
};

pub mod values;

pub struct Context {}

pub struct Script {
    // bytecode: Vec<u8>,
    body: Vec<Node>,
}

impl Script {
    pub fn new(code: &str) -> Self {
        Self {
            // bytecode: self.compile(),
            body: match Ast::new(code).build() {
                Ok(program) => match program {
                    Statement::Program(nodes) => nodes,
                },
                Err(()) => panic!(""),
            },
        }
    }

    // fn compile(&self, ...) -> Vec<u8> {}

    pub fn run_in_context(&self, context: &Context) -> Value {
        let nodes = self.body.clone();
        let mut last_value = Value::Undefined;
        for node in nodes {
            match node {
                Node::Expression(expression) => {
                    last_value = self.eval_expression(expression, context);
                }
                Node::Statement(_) => {}
            }
        }

        last_value
    }

    fn eval_expression(&self, expression: Expression, _context: &Context) -> Value {
        match expression {
            Expression::BinaryExpression(_, _, _) => {
                self.eval_binary_expression(expression, _context)
            }

            Expression::NumberLiteral(value) => Value::Number(value),
            Expression::StringLiteral(value) => Value::String(value),
        }
    }

    fn eval_binary_expression(&self, expression: Expression, _context: &Context) -> Value {
        match expression {
            Expression::BinaryExpression(left, right, op) => {
                let left = self.eval_expression(*left, _context);
                let right = self.eval_expression(*right, _context);

                let left = match left {
                    Value::Number(n) => n,
                    _ => return Value::Number(f32::NAN),
                };

                let right = match right {
                    Value::Number(n) => n,
                    _ => return Value::Number(f32::NAN),
                };

                match op {
                    Token::Plus => Value::Number(left + right),
                    Token::Minus => Value::Number(left - right),
                    Token::Multiply => Value::Number(left * right),
                    Token::Divide => Value::Number(left / right),
                    _ => panic!("Unknown math operator: {:?}", op),
                }
            }
            _ => panic!("Invalid binary expression"),
        }
    }
}
