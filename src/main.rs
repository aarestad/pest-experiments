use env_logger::Env;
use std::error::Error;
use std::fs;

use log::info;

mod parsers;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let unparsed_file = fs::read_to_string("config.ini")?;
    let properties = parsers::ini::parse_to_map(&unparsed_file)?;
    info!("ini result: {:#?}", properties);

    let simple_program = fs::read_to_string("prog1.simple")?;
    let simple_program_state = parsers::simple::parse(&simple_program)?;
    info!("simple program result state: {:#?}", simple_program_state);

    Ok(())
}
