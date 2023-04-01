use std::error::Error;
use std::fs;

mod parsers;

fn main() -> Result<(), Box<dyn Error>> {
    let unparsed_file = fs::read_to_string("config.ini")?;

    let properties = parsers::ini::parse_to_map(&unparsed_file)?;

    println!("{:#?}", properties);

    let simple_program = r#"
    x = 5
    y = x + 3
    z = y / 2
    "#;

    let simple_program_state = parsers::simple::parse(simple_program)?;
    println!("{:#?}", simple_program_state);

    Ok(())
}
