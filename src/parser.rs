use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alpha1, char, i64, multispace0},
    combinator::{map, opt},
    multi::many1,
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::interpreter::RuntimeValue;

// Atoms

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    String(String),
    Variable(String),
    Boolean(bool),
    Integer(i64),
    Double(f64),
}

impl Atom {
    pub fn to_string(&self) -> String {
        match self {
            Atom::Boolean(boolean) => boolean.to_string(),
            Atom::Integer(integer) => integer.to_string(),
            Atom::Double(float) => float.to_string(),
            Atom::String(string) => string.to_string(),
            Atom::Variable(var) => var.to_string(),
        }
    }
}

impl From<Atom> for Expression {
    fn from(atom: Atom) -> Self {
        Expression::Atom(atom)
    }
}

impl From<Atom> for RuntimeValue {
    fn from(atom: Atom) -> Self {
        match atom {
            Atom::Boolean(boolean) => RuntimeValue::Boolean(boolean),
            Atom::Integer(integer) => RuntimeValue::Integer(integer),
            Atom::Double(float) => RuntimeValue::Double(float),
            Atom::String(string) => RuntimeValue::String(string),
            Atom::Variable(var) => panic!("Cannot convert variable to RuntimeValue: {}", var),
        }
    }
}

fn parse_atom(input: &str) -> IResult<&str, Expression> {
    let parser = alt((
        parse_boolean,
        parse_double,
        parse_integer,
        parse_string,
        parse_variable,
    ));
    map(parser, |atom| atom.into())(input)
}

fn parse_boolean(input: &str) -> IResult<&str, Atom> {
    let parser = alt((tag("true"), tag("false")));
    map(parser, |boolean: &str| Atom::Boolean(boolean == "true"))(input)
}

fn parse_double(input: &str) -> IResult<&str, Atom> {
    if !input.contains(".") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }

    let parser = double;
    map(parser, |float| Atom::Double(float))(input)
}

fn parse_integer(input: &str) -> IResult<&str, Atom> {
    let parser = i64;
    map(parser, |integer| Atom::Integer(integer))(input)
}

fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |var: &str| Atom::Variable(var.to_string()))(input)
}

// Expressions

#[derive(Debug, PartialEq)]
pub enum Expression {
    SpellCast(Spell, Box<Option<Expression>>),
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    Atom(Atom),
}

#[derive(Debug, PartialEq)]
pub enum Spell {
    AvadaKedabra,
    Revelio,
    Periculum,
    Lumus,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    dbg!("parse_expression");
    dbg!(input);
    alt((parse_spell_cast, parse_binary_operation, parse_atom))(input)
}

pub fn parse_spell_cast(input: &str) -> IResult<&str, Expression> {
    // take until ; or ->
    let spell_parser = delimited(tag("~"), alpha0, opt(tag(" ")));
    let target_parser = parse_expression;
    let parser = tuple((spell_parser, opt(target_parser)));

    map(parser, |(spell, target)| match spell {
        "AvadaKedabra" => Expression::SpellCast(Spell::AvadaKedabra, Box::new(target)),
        "Revelio" => Expression::SpellCast(Spell::Revelio, Box::new(target)),
        "Periculum" => Expression::SpellCast(Spell::Periculum, Box::new(target)),
        "Lumus" => Expression::SpellCast(Spell::Lumus, Box::new(target)),
        _ => panic!("Wand broken: Unknown spell: {}", spell),
    })(input)
}

pub fn parse_binary_operation(input: &str) -> IResult<&str, Expression> {
    let (rest, (left, _, op, _, right)) = tuple((
        parse_atom,
        multispace0,
        parse_binary_operator,
        multispace0,
        parse_atom,
    ))(input)?;

    let expression = Expression::BinaryOperation(op, Box::new(left), Box::new(right));
    Ok((rest, expression))
}

pub fn parse_binary_operator(input: &str) -> IResult<&str, BinaryOperation> {
    alt((
        map(char('+'), |_| BinaryOperation::Plus),
        map(char('-'), |_| BinaryOperation::Minus),
        map(char('*'), |_| BinaryOperation::Times),
        map(char('/'), |_| BinaryOperation::Divide),
        map(tag("=="), |_| BinaryOperation::Equal),
        map(tag("!="), |_| BinaryOperation::NotEqual),
    ))(input)
}

// Statements

#[derive(Debug, PartialEq)]
pub enum Statement {
    VariableAssignment(String, Expression),
    ExpressionStatement(Expression),
    If(Expression, Vec<Statement>),
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    let parser_content = alt((
        parse_if,
        parse_variable_assignment,
        parse_expression_statement,
    ));

    preceded(multispace0, terminated(parser_content, multispace0))(input)
}

fn parse_variable_assignment(input: &str) -> IResult<&str, Statement> {
    let (rest, (var, _, _, _, atom)) = tuple((
        parse_variable,
        multispace0,
        char('='),
        multispace0,
        parse_expression,
    ))(input)?;

    let statement = Statement::VariableAssignment(var.to_string(), atom);
    Ok((rest, statement))
}

fn parse_expression_statement(input: &str) -> IResult<&str, Statement> {
    let (rest, expression) = terminated(parse_expression, multispace0)(input)?;
    let statement = Statement::ExpressionStatement(expression);
    Ok((rest, statement))
}

fn parse_if(input: &str) -> IResult<&str, Statement> {
    let parse_condition = preceded(multispace0, terminated(parse_expression, multispace0));
    let parse_block = delimited(char('{'), many1(parse_statement), char('}'));

    map(
        pair(preceded(tag("if"), parse_condition), parse_block),
        |(cond, true_block)| Statement::If(cond, true_block),
    )(input)
}

// Program

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Statement>);

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    map(many1(terminated(parse_statement, multispace0)), Program)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Atoms

    #[test]
    fn test_parse_string() {
        let input = "\"Hello, world!\"";
        let expected = Atom::String("Hello, world!".to_string());
        let (_, actual) = parse_string(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_variable() {
        let input = "foo";
        let expected = Atom::Variable("foo".to_string());
        let (_, actual) = parse_variable(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_boolean_true() {
        let input = "true";
        let expected = Atom::Boolean(true);
        let (_, actual) = parse_boolean(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_boolean_false() {
        let input = "false";
        let expected = Atom::Boolean(false);
        let (_, actual) = parse_boolean(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_double() {
        let input = "123.456";
        let expected = Atom::Double(123.456);
        let (_, actual) = parse_double(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_integer() {
        let input = "123";
        let expected = Atom::Integer(123);
        let (_, actual) = parse_integer(input).unwrap();
        assert_eq!(expected, actual);
    }

    // Expressions

    #[test]
    fn test_parse_statement_with_whitespaces() {
        let input = " ~AvadaKedabra ";
        let expected = Statement::ExpressionStatement(Expression::SpellCast(
            Spell::AvadaKedabra,
            Box::new(None),
        ));
        let (_, actual) = parse_statement(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast() {
        let input = "~AvadaKedabra";
        let expected = Expression::SpellCast(Spell::AvadaKedabra, Box::new(None));
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast_with_string() {
        let input = "~Revelio \"Hello, world!\"";
        let expected = Expression::SpellCast(
            Spell::Revelio,
            Box::new(Some(Atom::String("Hello, world!".to_string()).into())),
        );
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast_with_string_and_space() {
        let input = "~Revelio \"Hello, world!\" ";
        let expected = Expression::SpellCast(
            Spell::Revelio,
            Box::new(Some(Atom::String("Hello, world!".to_string()).into())),
        );
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_binary_operation() {
        let input = "\"Hello, \" + \"world!\"";
        let expected = Expression::BinaryOperation(
            BinaryOperation::Plus,
            Box::new(Atom::String("Hello, ".to_string()).into()),
            Box::new(Atom::String("world!".to_string()).into()),
        );
        let (_, actual) = parse_binary_operation(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_binary_operation_with_variable() {
        let input = "foo + \"bar\"";
        let expected = Expression::BinaryOperation(
            BinaryOperation::Plus,
            Box::new(Atom::Variable("foo".to_string()).into()),
            Box::new(Atom::String("bar".to_string()).into()),
        );
        let (_, actual) = parse_binary_operation(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_binary_operation_with_integer() {
        let input = "123 + 456";
        let expected = Expression::BinaryOperation(
            BinaryOperation::Plus,
            Box::new(Atom::Integer(123).into()),
            Box::new(Atom::Integer(456).into()),
        );
        let (_, actual) = parse_binary_operation(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_binary_operation_with_double() {
        let input = "123.456 + 456.789";
        let expected = Expression::BinaryOperation(
            BinaryOperation::Plus,
            Box::new(Atom::Double(123.456).into()),
            Box::new(Atom::Double(456.789).into()),
        );
        let (_, actual) = parse_binary_operation(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_binary_operation_with_boolean() {
        let input = "true == false";
        let expected = Expression::BinaryOperation(
            BinaryOperation::Equal,
            Box::new(Atom::Boolean(true).into()),
            Box::new(Atom::Boolean(false).into()),
        );
        let (_, actual) = parse_binary_operation(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_binary_operation_with_variable_and_integer() {
        let input = "foo + 4";
        let expected = Expression::BinaryOperation(
            BinaryOperation::Plus,
            Box::new(Atom::Variable("foo".to_string()).into()),
            Box::new(Atom::Integer(4).into()),
        );
        let (_, actual) = parse_binary_operation(input).unwrap();
        assert_eq!(expected, actual);
    }

    // Statements

    #[test]
    fn test_parse_variable_assignment() {
        let input = "foo = \"Hello, world!\"";
        let expected = Statement::VariableAssignment(
            "foo".to_string(),
            Atom::String("Hello, world!".to_string()).into(),
        );
        let (_, actual) = parse_variable_assignment(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_expression_statement() {
        let input = "~AvadaKedabra";
        let expected = Statement::ExpressionStatement(Expression::SpellCast(
            Spell::AvadaKedabra,
            Box::new(None),
        ));
        let (_, actual) = parse_expression_statement(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_expression_statement_with_string() {
        let input = "~Revelio \"Hello, world!\"";
        let expected = Statement::ExpressionStatement(Expression::SpellCast(
            Spell::Revelio,
            Box::new(Some(Atom::String("Hello, world!".to_string()).into())),
        ));
        let (_, actual) = parse_expression_statement(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_if() {
        let input = "if true { ~Revelio 4 }";
        let expected = Statement::If(
            Atom::Boolean(true).into(),
            vec![Statement::ExpressionStatement(Expression::SpellCast(
                Spell::Revelio,
                Box::new(Some(Atom::Integer(4).into())),
            ))],
        );
        let (_, actual) = parse_if(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_if_multiple_statements() {
        let input = "if 4 == 4 {
          ~Revelio 4 
          ~AvadaKedabra
        }";
        let expected = Statement::If(
            Expression::BinaryOperation(
                BinaryOperation::Equal,
                Box::new(Atom::Integer(4).into()),
                Box::new(Atom::Integer(4).into()),
            ),
            vec![
                Statement::ExpressionStatement(Expression::SpellCast(
                    Spell::Revelio,
                    Box::new(Some(Atom::Integer(4).into())),
                )),
                Statement::ExpressionStatement(Expression::SpellCast(
                    Spell::AvadaKedabra,
                    Box::new(None),
                )),
            ],
        );
        let (_, actual) = parse_if(input).unwrap();
        assert_eq!(expected, actual);
    }

    // Program

    #[test]
    fn test_parse_program() {
        let input = "~AvadaKedabra\n~Revelio \"Hello, world!\"";
        let expected = Program(vec![
            Statement::ExpressionStatement(Expression::SpellCast(
                Spell::AvadaKedabra,
                Box::new(None),
            )),
            Statement::ExpressionStatement(Expression::SpellCast(
                Spell::Revelio,
                Box::new(Some(Atom::String("Hello, world!".to_string()).into())),
            )),
        ]);
        let (_, actual) = parse_program(input).unwrap();
        assert_eq!(expected, actual);
    }
}
