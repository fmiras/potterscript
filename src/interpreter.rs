use core::panic;
use std::collections::HashMap;

use crate::parser::*;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Integer(i64),
    Double(f64),
    Boolean(bool),
    String(String),
}

pub struct Interpreter {
    variables: HashMap<String, RuntimeValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, program: Program) {
        for statement in program.0 {
            self.eval_statement(statement);
        }
    }

    fn eval_statement(&mut self, statement: Statement) {
        match statement {
            Statement::ExpressionStatement(expression) => {
                self.eval_expression(expression);
            }
            Statement::VariableAssignment(name, value) => {
                let evaluated_value = self.eval_expression(value);

                if let Some(evaluated_value) = evaluated_value {
                    self.variables.insert(name, evaluated_value);
                } else {
                    panic!("Cannot assign None to variable");
                }
            }
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Option<RuntimeValue> {
        match expression {
            Expression::SpellCast(spell, target) => self.eval_spell(spell, target),
            Expression::BinaryOperation(_, _, _) => {
                todo!("BinaryOperation");
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
                todo!("Lumus");
            }

            Spell::Revelio => match *target {
                Some(target) => {
                    println!("👀 Revelio: {:?}", self.eval_expression(target));
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
