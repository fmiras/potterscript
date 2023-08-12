use nom::{
    bytes::complete::{tag, take_until},
    character::complete::alpha0,
    combinator::{map, opt},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Atom {
    String(String),
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

#[derive(Debug, PartialEq)]
pub enum Spell {
    AvadaKedabra,
    Revelio,
    Periculum,
    Lumus,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    SpellCast(Spell, Option<Atom>),
}

pub fn parse_spell_cast(input: &str) -> IResult<&str, Expression> {
    // take until ; or ->
    let spell_parser = delimited(tag("~"), alpha0, opt(tag(" ")));
    let target_parser = parse_string;
    let parser = tuple((spell_parser, opt(target_parser)));

    map(parser, |(spell, target)| match spell {
        "AvadaKedabra" => Expression::SpellCast(Spell::AvadaKedabra, target),
        "Revelio" => Expression::SpellCast(Spell::Revelio, target),
        "Periculum" => Expression::SpellCast(Spell::Periculum, target),
        "Lumus" => Expression::SpellCast(Spell::Lumus, target),
        _ => panic!("Wand broken: Unknown spell: {}", spell),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let input = "\"Hello, world!\"";
        let expected = Atom::String("Hello, world!".to_string());
        let (_, actual) = parse_string(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast() {
        let input = "~AvadaKedabra";
        let expected = Expression::SpellCast(Spell::AvadaKedabra, None);
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast_with_string() {
        let input = "~Revelio \"Hello, world!\"";
        let expected = Expression::SpellCast(
            Spell::Revelio,
            Some(Atom::String("Hello, world!".to_string())),
        );
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast_with_string_and_space() {
        let input = "~Revelio \"Hello, world!\" ";
        let expected = Expression::SpellCast(
            Spell::Revelio,
            Some(Atom::String("Hello, world!".to_string())),
        );
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }
}
