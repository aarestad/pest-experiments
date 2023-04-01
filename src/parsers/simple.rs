use std::collections::HashMap;

use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parsers/simple.pest"]
struct SimpleParser;

type SimpleProgramState = HashMap<String, i64>;

#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<SimpleProgramState, Error<Rule>> {
    let file = SimpleParser::parse(Rule::file, input)?
        .next()
        .expect("bad parsing output");

    let mut program_state: SimpleProgramState = HashMap::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::statement => process_statement(&mut line.into_inner(), &mut program_state)?,
            Rule::EOI => (),
            _ => return Err(unexpected_type(line)),
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
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("unexpected type: {}", stmt_type),
                    },
                    stmt_type.as_span(),
                ))
            }
        }
    }

    Ok(())
}

fn process_assign(
    assign_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    // { variable ~ "=" ~ expression }
    let variable = assign_rule.next().expect("invalid parse");

    if variable.as_rule() != Rule::variable {
        return Err(unexpected_type(variable));
    }

    let mut expression = assign_rule.next().expect("invalid parse");

    let result = evaluate_expression(&mut expression, program_state)?;
    program_state.insert(variable.as_str().into(), result);

    Ok(())
}

fn process_while(
    while_rule: &mut Pairs<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<(), Error<Rule>> {
    Ok(())
}

fn evaluate_expression(
    expression_rule: &mut Pair<Rule>,
    program_state: &mut SimpleProgramState,
) -> Result<i64, Error<Rule>> {
    Ok(0)
}

fn unexpected_type(type_pair: Pair<Rule>) -> Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: format!("unexpected type: {}", type_pair),
        },
        type_pair.as_span(),
    )
}
