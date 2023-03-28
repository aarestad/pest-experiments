use std::fs;
use std::io::Error;

mod parsers;

fn main() -> Result<(), Error> {
    let unparsed_file = fs::read_to_string("config.ini")?;

    let properties = parsers::ini::parse_to_map(&unparsed_file);

    println!("{:#?}", properties);

    Ok(())
}
