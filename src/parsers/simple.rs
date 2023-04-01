use std::collections::HashMap;

use core::cmp::Ord;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::ops::{Add, Div, Mul, Sub};

use log::debug;

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
            Rule::statement => eval_statement(&mut line.into_inner(), &mut program_state)?,
            Rule::EOI => (),
            _ => panic!("invalid parse"),
        }
    }

    Ok(program_state)
}

fn eval_statement(
    statement: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    for stmt_type in statement.into_iter() {
        match stmt_type.as_rule() {
            Rule::while_stmt => eval_while(&mut stmt_type.into_inner(), program_state)?,
            Rule::assign_stmt => eval_assign(&mut stmt_type.into_inner(), program_state)?,
            _ => panic!("invalid parse"),
        }
    }

    Ok(())
}

fn eval_assign(
    assign_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    debug!("{}", &assign_rule.as_str());
    let variable = assign_rule.next().expect("invalid parse");
    let expression = assign_rule.next().expect("invalid parse");

    let value = eval_expression(&mut expression.into_inner(), program_state)?;

    program_state.insert(variable.as_str().into(), value);

    Ok(())
}

fn eval_while(
    while_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    let test_expr = while_rule.next().expect("invalid parse").into_inner();
    let statements: Vec<Pair<Rule>> = while_rule.collect();

    debug!("{}", test_expr.as_str());

    for s in &statements {
        debug!("{}", s.as_str());
    }

    loop {
        let test_result_val = eval_expression(&mut test_expr.clone(), program_state)?;
        let test_result = test_result_val.as_bool().expect("unexpected type of val");

        if *test_result {
            for statement in &statements {
                eval_statement(&mut statement.clone().into_inner(), program_state)?;
            }
        } else {
            return Ok(());
        }
    }
}

fn eval_expression(
    expression_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    debug!("{}", &expression_rule.as_str());
    let term = expression_rule
        .next()
        .expect(format!("invalid parse: {:?}", expression_rule).as_str());

    let expression_op = expression_rule.next();

    let op_pair = match expression_op {
        Some(op) => op,
        None => return eval_term(&mut term.into_inner(), program_state),
    };

    let op_category = op_pair.as_rule();
    let op = op_pair.clone().into_inner().next().expect("invalid parse");

    let trailing_expr = expression_rule.next().expect("invalid parse");

    match op_category {
        Rule::expression_op => {
            let op_fn = match op.as_rule() {
                Rule::add => i64::add,
                Rule::subtract => i64::sub,
                _ => panic!("invalid parse: {:?}", op)
            };

            return process_binary_i64_op(op_fn, term, trailing_expr, program_state)
        },
        Rule::boolean_expression_op => {
            let op_fn = match op.as_rule() {
                Rule::lt => i64::lt,
                Rule::le => i64::le,
                Rule::eq => i64::eq,
                Rule::ge => i64::ge,
                Rule::gt => i64::gt,
                _ => panic!("invalid parse: {:?}", op)
            };

            return process_binary_bool_op(op_fn, term, trailing_expr, program_state)
        },
        _ => panic! ("invalid parse: {}", op_pair)
    }
}

fn eval_term(
    term_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let factor = term_rule.next().expect("invalid parse");
    let term_op = term_rule.next();

    let op_pair = match term_op {
        Some(op) => op,
        None => return eval_factor(&mut factor.into_inner(), program_state),
    };

    let op = op_pair.into_inner().next().expect("invalid parse");
    let trailing_term = term_rule.next().expect("invalid parse");

    match op.as_rule() {
        Rule::mul => process_binary_i64_op(i64::mul, factor, trailing_term, program_state),
        Rule::div => process_binary_i64_op(i64::div, factor, trailing_term, program_state),
        _ => panic!("invalid parse: {:?}", op.as_rule()),
    }
}

fn eval_factor(
    factor_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let val_or_expr = factor_rule.next().expect("invalid parse");

    match val_or_expr.as_rule() {
        Rule::expression => eval_expression(&mut val_or_expr.into_inner(), program_state),

        Rule::number => Ok(Val::Integer(
            val_or_expr.as_str().parse().expect("invalid parse"),
        )),

        Rule::boolean => Ok(Val::Boolean(val_or_expr.as_str() == "true")),

        Rule::variable => {
            let result = program_state.get(val_or_expr.as_str());

            match result {
                Some(&v) => Ok(v),
                None => Err(custom_error("unrecognized var name", val_or_expr)),
            }
        }
        _ => panic!("invalid parse"),
    }
}

// TODO: make this more generic to merge with process_binary_bool_op
fn process_binary_i64_op(
    op: fn(i64, i64) -> i64,
    lhs_pair: Pair<Rule>,
    rhs_pair: Pair<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let lhs_val = match lhs_pair.as_rule() {
        Rule::factor => eval_factor(&mut lhs_pair.clone().into_inner(), program_state)?,
        Rule::term => eval_term(&mut lhs_pair.clone().into_inner(), program_state)?,
        _ => panic!("invalid parse"),
    };

    let lhs = lhs_val
        .as_integer()
        .ok_or(custom_error("unexpected type of val", lhs_pair))?;

    let rhs_val = match rhs_pair.as_rule() {
        Rule::expression => eval_expression(&mut rhs_pair.clone().into_inner(), program_state)?,
        Rule::term => eval_term(&mut rhs_pair.clone().into_inner(), program_state)?,
        _ => panic!("invalid parse"),
    };

    let rhs = rhs_val
        .as_integer()
        .ok_or(custom_error("unexpected tyoe of val", rhs_pair))?;

    Ok(Val::Integer(op(*lhs, *rhs)))
}

fn process_binary_bool_op(
    op: fn(&i64, &i64) -> bool,
    lhs_pair: Pair<Rule>,
    rhs_pair: Pair<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<Val, Error<Rule>> {
    let lhs_val = match lhs_pair.as_rule() {
        Rule::factor => eval_factor(&mut lhs_pair.clone().into_inner(), program_state)?,
        Rule::term => eval_term(&mut lhs_pair.clone().into_inner(), program_state)?,
        _ => panic!("invalid parse"),
    };

    let lhs = lhs_val
        .as_integer()
        .ok_or(custom_error("unexpected type of val", lhs_pair))?;

    let rhs_val = match rhs_pair.as_rule() {
        Rule::expression => eval_expression(&mut rhs_pair.clone().into_inner(), program_state)?,
        Rule::term => eval_term(&mut rhs_pair.clone().into_inner(), program_state)?,
        _ => panic!("invalid parse"),
    };

    let rhs = rhs_val
        .as_integer()
        .ok_or(custom_error("unexpected tyoe of val", rhs_pair))?;

    Ok(Val::Boolean(op(lhs, rhs)))
}

fn custom_error(msg: &str, rule: Pair<Rule>) -> Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: format!("{}: {}", msg, rule.as_str()),
        },
        rule.as_span(),
    )
}
