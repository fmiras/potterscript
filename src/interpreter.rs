use crate::parser::*;

fn eval_statement(statement: Statement) {
    match statement {
        Statement::ExpressionStatement(expression) => {
            eval_expression(expression);
        }
    }
}

fn eval_expression(expression: Expression) {
    match expression {
        Expression::SpellCast(spell, target) => {
            eval_spell(spell, target);
        }
    }
}

fn eval_spell(spell: Spell, target: Option<Atom>) {
    match spell {
        Spell::AvadaKedabra => {
            panic!();
        }
        Spell::Periculum => {
            println!("🔥🔥🔥🔥🔥🔥🔥🔥🔥");
        }
        Spell::Lumus => {
            todo!("Lumus")
        }
        Spell::Revelio => {
            println!("👀 Revelio: {:?}", target);
        }
    }
}

pub fn eval(program: Program) {
    for statement in program.0 {
        eval_statement(statement);
    }
}
