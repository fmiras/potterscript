use core::{panic, time};
use std::collections::HashMap;
use std::{fmt, ops, process, thread};

use colored::Colorize;
use rand::Rng;

use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Integer(i64),
    Double(f64),
    Boolean(bool),
    String(String),
    HogwartsHouse(HogwartsHouse),
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Integer(value) => write!(f, "{}", value),
            RuntimeValue::Double(value) => write!(f, "{}", value),
            RuntimeValue::Boolean(value) => write!(f, "{}", value),
            RuntimeValue::String(value) => write!(f, "{}", value),
            RuntimeValue::HogwartsHouse(value) => write!(f, "{:?}", value),
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
    constants: HashMap<String, RuntimeValue>,
    quidditch: bool,
    is_lumos_casted: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashMap::new(),
            quidditch: false,
            is_lumos_casted: false,
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
                if self.constants.contains_key(&name) {
                    panic!("Cannot re-assign a constant!");
                }

                // dbg!(format!("VariableAssignment: {:?} = {:?}", name, value));
                let evaluated_value = self.eval_expression(value);

                if let Some(evaluated_value) = evaluated_value {
                    self.variables.insert(name, evaluated_value);
                } else {
                    panic!("Cannot assign None to variable");
                }
            }
            Statement::ExpressionStatement(expression) => {
                // dbg!(format!("ExpressionStatement: {:?}", expression));
                self.eval_expression(expression);
            }
            Statement::If(condition, true_block, else_block) => {
                // dbg!(format!("If: {:?} {{ ... }}", condition));
                if let Some(RuntimeValue::Boolean(true)) = self.eval_expression(condition) {
                    for statement in true_block {
                        self.eval_statement(statement);
                    }
                } else {
                    for statement in else_block {
                        self.eval_statement(statement);
                    }
                }
            }
            Statement::Quidditch(block) => {
                // dbg!(format!("Quidditch: {:?} {{ ... }}", condition));
                self.quidditch = true;

                // arc
                while self.quidditch {
                    self.eval(Program(block.clone()))
                }
            }
            Statement::Snitch => {
                // dbg!("Snitch");
                self.quidditch = false;
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
            Expression::Comment(_) => None,
            Expression::SortingHat => {
                let houses = vec![
                    HogwartsHouse::Gryffindor,
                    HogwartsHouse::Hufflepuff,
                    HogwartsHouse::Ravenclaw,
                    HogwartsHouse::Slytherin,
                ];
                let mut rng = rand::thread_rng();
                let index: usize = rng.gen_range(0..=3);

                let random_house = houses[index];
                Some(RuntimeValue::HogwartsHouse(random_house.clone()))
            }
        }
    }

    fn eval_spell(
        &mut self,
        spell: Spell,
        target: Box<Option<Expression>>,
    ) -> Option<RuntimeValue> {
        match spell {
            Spell::AvadaKedabra => process::exit(0),
            Spell::Inmobolus => match *target {
                Some(Expression::Atom(Atom::Integer(number))) => {
                    let ms = time::Duration::from_millis(number as u64);
                    thread::sleep(ms);
                    None
                }
                _ => None,
            },
            Spell::Incendio => match *target {
                Some(Expression::Atom(Atom::Variable(var_name))) => {
                    let value = self
                        .variables
                        .get(&var_name)
                        .cloned()
                        .expect(format!("Variable {} not found", var_name).as_str());
                    match value {
                        RuntimeValue::String(string) => {
                            self.variables
                                .insert(var_name, RuntimeValue::String(string + "🔥"));
                        }
                        _ => panic!("Cannot Incendio {:?}", value),
                    }
                    None
                }
                Some(Expression::Atom(Atom::String(string))) => {
                    Some(RuntimeValue::String(string + "🔥"))
                }
                _ => None,
            },
            Spell::Aguamenti => RuntimeValue::String("💦".to_string()).into(),
            Spell::OculusReparo => RuntimeValue::String("👓".to_string()).into(),
            Spell::Serpensortia => RuntimeValue::String("🐍".to_string()).into(),
            Spell::Periculum => {
                println!("🔥🔥🔥🔥🔥🔥🔥🔥🔥");
                None
            }
            Spell::Lumos => {
                self.is_lumos_casted = true;
                None
            }
            Spell::Nox => {
                self.is_lumos_casted = false;
                None
            }
            Spell::Engorgio => match *target {
                Some(Expression::Atom(Atom::Variable(var_name))) => {
                    let value = self
                        .variables
                        .get(&var_name)
                        .cloned()
                        .expect(format!("Variable {} not found", var_name).as_str());
                    match value {
                        RuntimeValue::Integer(value) => {
                            self.variables
                                .insert(var_name, RuntimeValue::Integer(value + 1));
                        }
                        RuntimeValue::Double(value) => {
                            self.variables
                                .insert(var_name, RuntimeValue::Double(value + 1.0));
                        }
                        RuntimeValue::String(string) => {
                            self.variables.insert(
                                var_name,
                                RuntimeValue::String(string.to_ascii_uppercase()),
                            );
                        }
                        _ => panic!("Cannot increment {:?}", value),
                    }
                    None
                }
                _ => None,
            },
            Spell::Reducio => match *target {
                Some(Expression::Atom(Atom::Variable(var_name))) => {
                    let value = self
                        .variables
                        .get(&var_name)
                        .cloned()
                        .expect(format!("Variable {} not found", var_name).as_str());
                    match value {
                        RuntimeValue::Integer(value) => {
                            self.variables
                                .insert(var_name, RuntimeValue::Integer(value - 1));
                        }
                        RuntimeValue::Double(value) => {
                            self.variables
                                .insert(var_name, RuntimeValue::Double(value - 1.0));
                        }
                        RuntimeValue::String(string) => {
                            self.variables.insert(
                                var_name,
                                RuntimeValue::String(string.to_ascii_lowercase()),
                            );
                        }
                        _ => panic!("Cannot decrement {:?}", value),
                    }
                    None
                }
                _ => None,
            },
            Spell::Obliviate => match *target {
                Some(Expression::Atom(Atom::Variable(var_name))) => {
                    self.variables.remove(&var_name);
                    None
                }
                _ => None,
            },
            Spell::Revelio => match *target {
                Some(target) => {
                    let mut string_target: String = self
                        .eval_expression(target)
                        .unwrap_or(RuntimeValue::String("".to_string()))
                        .to_string();
                    if self.is_lumos_casted {
                        string_target = string_target.black().on_white().to_string();
                    }
                    println!("{}", string_target);
                    None
                }
                None => None,
            },
            Spell::PetrificusTotalus => match *target {
                Some(Expression::Atom(Atom::Variable(var_name))) => {
                    let value = self.variables.remove(&var_name);
                    if let Some(value) = value {
                        self.constants.insert(var_name, value);
                    }
                    None
                }
                _ => None,
            },
            Spell::WingardiumLeviosa => match *target {
                Some(Expression::Atom(Atom::Variable(var_name))) => {
                    let value = self
                        .variables
                        .get(&var_name)
                        .cloned()
                        .expect(format!("Variable {} not found", var_name).as_str());
                    match value {
                        RuntimeValue::String(string) => {
                            self.variables
                                .insert(var_name, RuntimeValue::String(string + "\n"));
                        }
                        _ => panic!("Cannot WingardiumLeviosa {:?}", value),
                    }
                    None
                }
                Some(Expression::Atom(Atom::String(string))) => {
                    Some(RuntimeValue::String(string + "\n"))
                }
                _ => None,
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
