use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
pub enum Atom {
    String(String),
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

#[derive(Debug)]
pub enum Spell {
    AvadaKedabra,
}

#[derive(Debug)]
pub enum Expression {
    SpellCast(Spell, Option<Atom>),
}

pub fn parse_spell_cast(input: &str) -> IResult<&str, Expression> {
    let parser = delimited(tag("~"), take_until(";"), tag(";"));
    map(parser, |spell_name: &str| match spell_name {
        "Avada Kedabra" => Expression::SpellCast(Spell::AvadaKedabra, None),
        _ => panic!("Unknown spell: {}", spell_name),
    })(input)
}
