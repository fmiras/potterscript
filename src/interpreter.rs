use core::panic;
use std::collections::HashMap;

use crate::parser::*;

pub struct Interpreter {
    variables: HashMap<String, Atom>,
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
                let evaluated_value = self.eval_atom(value);
                self.variables.insert(name, evaluated_value);
            }
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Option<Atom> {
        match expression {
            Expression::SpellCast(spell, target) => self.eval_spell(spell, target),
            Expression::BinaryOperation(_, _, _) => {
                panic!("BinaryOperation");
            }
        }
    }

    fn eval_spell(&self, spell: Spell, target: Option<Atom>) -> Option<Atom> {
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

            Spell::Revelio => match target {
                Some(target) => {
                    println!("👀 Revelio: {:?}", self.eval_atom(target));
                    None
                }
                None => None,
            },
        }
    }

    fn eval_atom(&self, atom: Atom) -> Atom {
        match atom {
            Atom::String(_) => atom,
            Atom::Variable(var_name) => self
                .variables
                .get(&var_name)
                .cloned()
                .expect(format!("Variable {} not found", var_name).as_str()),
            Atom::Boolean(_) => atom,
            Atom::Integer(_) => atom,
            Atom::Double(_) => atom,
        }
    }
}
