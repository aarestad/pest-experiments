use env_logger::Env;
use std::error::Error;
use std::fs;

use log::{error, info};

mod parsers;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let simple_program = fs::read_to_string("prog1.simple")?;
    let simple_program_state_result = parsers::simple::parse(&simple_program);

    match simple_program_state_result {
        Ok(simple_program_state) => {
            info!("simple program result state: {:#?}", simple_program_state)
        }
        Err(e) => error!("parsing error: {}", e)
    }

    Ok(())
}
