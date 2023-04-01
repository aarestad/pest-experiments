use std::collections::HashMap;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parsers/simple.pest"]
struct SimpleParser;

type SimpleProgramState = HashMap<String, Val>;

#[derive(Debug, Clone, Copy)]
pub enum Val {
    Integer(i64),
    Boolean(bool),
}

impl Val {
    fn as_integer(&self) -> Option<&i64> {
        match self {
            Val::Integer(v) => Some(v),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn as_bool(&self) -> Option<&bool> {
        match self {
            Val::Boolean(v) => Some(v),
            _ => None,
        }
    }
}

pub fn parse(input: &str) -> Result<SimpleProgramState, Error<Rule>> {
    let file = SimpleParser::parse(Rule::file, input)?
        .next()
        .expect("invalid parse");

    let mut program_state: SimpleProgramState = HashMap::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::statement => process_statement(&mut line.into_inner(), &mut program_state)?,
            Rule::EOI => (),
            _ => panic!("invalid parse"),
        }
    }

    Ok(program_state)
}

fn process_statement(
    statement: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    for stmt_type in statement.into_iter() {
        match stmt_type.as_rule() {
            Rule::while_stmt => process_while(&mut stmt_type.into_inner(), program_state)?,
            Rule::assign => process_assign(&mut stmt_type.into_inner(), program_state)?,
            _ => panic!("invalid parse"),
        }
    }

    Ok(())
}

fn process_assign(
    assign_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    let variable = assign_rule.next().expect("invalid parse");
    let expression = assign_rule.next().expect("invalid parse");

    let value = evaluate_expression(&mut expression.into_inner(), program_state)?;

    program_state.insert(variable.as_str().into(), value);

    Ok(())
}

fn process_while(
    _while_rule: &mut Pairs<Rule>,
    _program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    Ok(())
}

fn evaluate_expression(
    expression_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let term = expression_rule.next().expect("invalid parse");

    let expression_op = expression_rule.next();

    let op_pair = match expression_op {
        Some(op) => op,
        None => return evaluate_term(&mut term.into_inner(), program_state),
    };

    let op = op_pair.into_inner().next().expect("invalid parse");

    let trailing_expr = expression_rule.next().expect("invalid parse");

    match op.as_rule() {
        Rule::add => Ok(Val::Integer(
            evaluate_term(&mut term.into_inner(), program_state)?
                .as_integer()
                .expect("unexpected type of val")
                + evaluate_expression(&mut trailing_expr.into_inner(), program_state)?
                    .as_integer()
                    .expect("unexpected type of val"),
        )),
        Rule::subtract => Ok(Val::Integer(
            evaluate_term(&mut term.into_inner(), program_state)?
                .as_integer()
                .expect("unexpected types of val")
                - evaluate_expression(&mut trailing_expr.into_inner(), program_state)?
                    .as_integer()
                    .expect("unexpected type"),
        )),
        _ => {
            panic!("invalid parse: {:?}", op.as_rule())
        },
    }
}

fn evaluate_term(
    term_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let factor = term_rule.next().expect("invalid parse");
    let term_op = term_rule.next();

    let op_pair = match term_op {
        Some(op) => op,
        None => return evaluate_factor(&mut factor.into_inner(), program_state),
    };

    let op = op_pair.into_inner().next().expect("invalid parse");

    let trailing_term = term_rule.next().expect("invalid parse");

    match op.as_rule() {
        Rule::mul => Ok(Val::Integer(
            evaluate_factor(&mut factor.into_inner(), program_state)?
                .as_integer()
                .expect("unexpected type of val")
                * evaluate_term(&mut trailing_term.into_inner(), program_state)?
                    .as_integer()
                    .expect("unexpect type of val"),
        )),
        Rule::div => Ok(Val::Integer(
            evaluate_factor(&mut factor.into_inner(), program_state)?
                .as_integer()
                .expect("unexpected type of val")
                / evaluate_term(&mut trailing_term.into_inner(), program_state)?
                    .as_integer()
                    .expect("unexpected type of val"),
        )),
        _ => panic!("invalid parse: {:?}", op.as_rule()),
    }
}

fn evaluate_factor(
    factor_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let val_or_expr = factor_rule.next().expect("invalid parse");

    match val_or_expr.as_rule() {
        Rule::expression => evaluate_expression(&mut val_or_expr.into_inner(), program_state),
        Rule::number => Ok(Val::Integer(
            val_or_expr.as_str().parse().expect("invalid number"),
        )),
        Rule::boolean => Ok(Val::Boolean(val_or_expr.as_str() == "true")),
        Rule::variable => Ok(*program_state
            .get(val_or_expr.as_str())
            .expect("unrecognized var name")),
        _ => panic!("invalid parse"),
    }
}
