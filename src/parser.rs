use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::{delimited, tuple},
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
pub enum Expression {
    SpellCast(String, Atom),
}

pub fn parse_spell_cast(input: &str) -> IResult<&str, Expression> {
    let parse_spell = delimited(tag("~"), take_until("->"), tag("->"));
    let parse_target = delimited(tag("->"), parse_string, tag(";"));
    let parser = tuple((parse_spell, parse_target));
    map(parser, |(spell, target)| {
        Expression::SpellCast(spell.to_string(), target)
    })(input)
}
