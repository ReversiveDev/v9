use self::values::Value;

use super::lexer::{
    ast::{Ast, Expression, Node},
    Token,
};

pub mod values;

pub struct Context {}

pub struct Script {
    // bytecode: Vec<u8>,
    ast: Ast,
}

impl Script {
    pub fn new(code: &str) -> Self {
        Self {
            // bytecode: self.compile(),
            ast: Ast::new(code),
        }
    }

    // fn compile(&self, ...) -> Vec<u8> {}

    pub fn run_in_context(&mut self, context: &Context) -> Value {
        let mut last_value = Value::Undefined;
        while let Some(node) = self.ast.next() {
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
