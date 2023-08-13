use core::panic;
use std::collections::HashMap;
use std::{fmt, ops};

use colored::Colorize;

use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Integer(i64),
    Double(f64),
    Boolean(bool),
    String(String),
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Integer(value) => write!(f, "{}", value),
            RuntimeValue::Double(value) => write!(f, "{}", value),
            RuntimeValue::Boolean(value) => write!(f, "{}", value),
            RuntimeValue::String(value) => write!(f, "{}", value),
        }
    }
}

impl ops::Add for RuntimeValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_value = self.clone();
        let rhs_value = rhs.clone();

        match (self, rhs) {
            (RuntimeValue::Integer(left), RuntimeValue::Integer(right)) => {
                RuntimeValue::Integer(left + right)
            }
            (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                RuntimeValue::Double(left + right)
            }
            (RuntimeValue::String(left), RuntimeValue::String(right)) => {
                RuntimeValue::String(left + &right)
            }
            _ => panic!("Cannot add {:?} and {:?}", self_value, rhs_value),
        }
    }
}

impl ops::Sub for RuntimeValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_value = self.clone();
        let rhs_value = rhs.clone();

        match (self, rhs) {
            (RuntimeValue::Integer(left), RuntimeValue::Integer(right)) => {
                RuntimeValue::Integer(left - right)
            }
            (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                RuntimeValue::Double(left - right)
            }
            _ => panic!("Cannot subtract {:?} and {:?}", self_value, rhs_value),
        }
    }
}

impl ops::Mul for RuntimeValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_value = self.clone();
        let rhs_value = rhs.clone();

        match (self, rhs) {
            (RuntimeValue::Integer(left), RuntimeValue::Integer(right)) => {
                RuntimeValue::Integer(left * right)
            }
            (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                RuntimeValue::Double(left * right)
            }
            _ => panic!("Cannot multiply {:?} and {:?}", self_value, rhs_value),
        }
    }
}

impl ops::Div for RuntimeValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let self_value = self.clone();
        let rhs_value = rhs.clone();

        match (self, rhs) {
            (RuntimeValue::Integer(left), RuntimeValue::Integer(right)) => {
                RuntimeValue::Integer(left / right)
            }
            (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                RuntimeValue::Double(left / right)
            }
            _ => panic!("Cannot divide {:?} and {:?}", self_value, rhs_value),
        }
    }
}

impl ops::Not for RuntimeValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            RuntimeValue::Boolean(value) => RuntimeValue::Boolean(!value),
            _ => panic!("Cannot negate {:?}", self),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, RuntimeValue>,
    is_lumus_casted: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            is_lumus_casted: false,
        }
    }

    pub fn eval(&mut self, program: Program) {
        for statement in program.0 {
            self.eval_statement(statement);
        }
    }

    fn eval_statement(&mut self, statement: Statement) {
        match statement {
            Statement::VariableAssignment(name, value) => {
                dbg!(format!("VariableAssignment: {:?} = {:?}", name, value));
                let evaluated_value = self.eval_expression(value);

                if let Some(evaluated_value) = evaluated_value {
                    self.variables.insert(name, evaluated_value);
                } else {
                    panic!("Cannot assign None to variable");
                }
            }
            Statement::ExpressionStatement(expression) => {
                dbg!(format!("ExpressionStatement: {:?}", expression));
                self.eval_expression(expression);
            }
            Statement::If(condition, true_block) => {
                dbg!(format!("If: {:?} {{ ... }}", condition));
                if let Some(RuntimeValue::Boolean(true)) = self.eval_expression(condition) {
                    for statement in true_block {
                        self.eval_statement(statement);
                    }
                }
            }
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Option<RuntimeValue> {
        match expression {
            Expression::SpellCast(spell, target) => self.eval_spell(spell, target),
            Expression::BinaryOperation(operation, left, right) => {
                match (self.eval_expression(*left), self.eval_expression(*right)) {
                    (Some(left), Some(right)) => match operation {
                        BinaryOperation::Plus => Some(left + right),
                        BinaryOperation::Minus => Some(left - right),
                        BinaryOperation::Times => Some(left * right),
                        BinaryOperation::Divide => Some(left / right),
                        BinaryOperation::Equal => Some(RuntimeValue::Boolean(left == right)),
                        BinaryOperation::NotEqual => Some(RuntimeValue::Boolean(left != right)),
                    },
                    _ => None,
                }
            }
            Expression::Atom(atom) => Some(self.eval_atom(atom)),
        }
    }

    fn eval_spell(
        &mut self,
        spell: Spell,
        target: Box<Option<Expression>>,
    ) -> Option<RuntimeValue> {
        match spell {
            Spell::AvadaKedabra => {
                panic!();
            }
            Spell::Periculum => {
                println!("🔥🔥🔥🔥🔥🔥🔥🔥🔥");
                None
            }
            Spell::Lumus => {
                self.is_lumus_casted = true;
                None
            }

            Spell::Revelio => match *target {
                Some(target) => {
                    let mut string_target: String = self
                        .eval_expression(target)
                        .unwrap_or(RuntimeValue::String("".to_string()))
                        .to_string();
                    if self.is_lumus_casted {
                        string_target = string_target.black().on_white().to_string();
                    }
                    println!("{}", string_target);
                    None
                }
                None => None,
            },
        }
    }

    fn eval_atom(&self, atom: Atom) -> RuntimeValue {
        match atom {
            Atom::Variable(var_name) => self
                .variables
                .get(&var_name)
                .cloned()
                .expect(format!("Variable {} not found", var_name).as_str()),
            _ => atom.into(),
        }
    }
}
