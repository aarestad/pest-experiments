use env_logger::Env;
use std::error::Error;
use std::fs;

use log::{error, info};
use pest::error::{ErrorVariant, LineColLocation};

mod parsers;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let simple_program = fs::read_to_string("prog1.simple")?;
    let simple_program_state_result = parsers::simple::parse(&simple_program);

    match simple_program_state_result {
        Ok(simple_program_state) => {
            info!("simple program result state: {:#?}", simple_program_state)
        }
        Err(e) => match e.variant {
            ErrorVariant::CustomError { message } => {
                let (start, end) = match e.line_col {
                    LineColLocation::Span(start, _) => start,
                    LineColLocation::Pos(p) => p,
                };

                error!("runtime error: {} at line {} col {}", message, start, end)
            }
            ErrorVariant::ParsingError {
                positives,
                negatives,
            } => error!(
                "parsing error: positives={:?}, negatives={:?}",
                positives, negatives
            ),
        },
    }

    Ok(())
}
