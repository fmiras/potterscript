use crate::parser::*;

pub fn eval(expression: Expression) {
    match expression {
        Expression::SpellCast(Spell::AvadaKedabra, _) => {
            panic!();
        }
        Expression::SpellCast(Spell::Periculum, _) => {
            println!("🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥");
        }
        Expression::SpellCast(Spell::Lumus, _) => {
            todo!("Lumus")
        }
        Expression::SpellCast(Spell::Revelio, Some(target)) => {
            println!("👀 Revelio: {:?}", target);
        }
        Expression::SpellCast(_, None) => {
            panic!("Wand broken: {:?} requires target", expression);
        }
        _ => panic!("Unknown expression: {:?}", expression),
    }
}
