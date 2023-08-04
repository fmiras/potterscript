use crate::parser::*;

pub fn eval(expression: Expression) -> Expression {
    match expression {
        Expression::SpellCast(spell, None) => {
            // show this if truth serum: println!("Casting spell: {:?}", spell);
            match spell {
                Spell::AvadaKedabra => {
                    panic!()
                }
            }
        }
        _ => panic!("Unknown expression: {:?}", expression),
    }
}
