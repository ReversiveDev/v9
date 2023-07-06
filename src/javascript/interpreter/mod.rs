use self::values::Value;

use super::lexer::{
    ast::{Ast, Expression, Node, Statement},
    Token,
};

pub mod values;

#[derive(Debug)]
pub struct Context {
    variables: Vec<(String, Value)>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn declare_variable(&mut self, name: &str, value: Value) {
        if self.has_variable(name) {
            unreachable!("Variable already declared");
        } else {
            self.set_variable(name, value);
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.variables.push((name.to_string(), value));
    }

    pub fn get_variable(&self, name: &str) -> Value {
        if self.has_variable(name) {
            self.variables
                .iter()
                .find(|(n, _)| n == name)
                .unwrap()
                .1
                .clone()
        } else {
            Value::Undefined
        }
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.iter().any(|(n, _)| n == name)
    }
}

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

    pub fn run_in_context(&mut self, context: &mut Context) -> Value {
        let mut last_value = Value::Undefined;
        while let Some(node) = self.ast.next() {
            match node {
                Node::Expression(expression) => {
                    last_value = self.eval_expression(expression, context);
                }
                Node::Statement(statement) => {
                    last_value = self.eval_statement(statement, context);
                }
            }
        }

        last_value
    }

    fn eval_statement(&self, statement: Statement, context: &mut Context) -> Value {
        match statement {
            Statement::VariableDeclaration(statements) => {
                for statement in statements {
                    self.eval_statement(statement, context);
                }
            }
            Statement::VariableDeclarator(identifier, initializer) => {
                if let Some(initializer) = *initializer {
                    context.declare_variable(
                        identifier.as_str(),
                        self.eval_expression(initializer, context),
                    );
                }
            }
        }

        Value::Undefined
    }

    fn eval_expression(&self, expression: Expression, context: &Context) -> Value {
        match expression {
            Expression::BinaryExpression(_, _, _) => {
                self.eval_binary_expression(expression, context)
            }
            Expression::NumberLiteral(value) => Value::Number(value),
            Expression::StringLiteral(value) => Value::String(value),
            Expression::Identifier(value) => context.get_variable(value.as_str()),
        }
    }

    fn eval_binary_expression(&self, expression: Expression, context: &Context) -> Value {
        match expression {
            Expression::BinaryExpression(left, right, op) => {
                let left = self.eval_expression(*left, context);
                let right = self.eval_expression(*right, context);

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
