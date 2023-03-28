use std::fs;

mod parsers;

fn main() {
    let unparsed_file = fs::read_to_string("config.ini").expect("cannot read file");

    let properties = parsers::ini::parse(&unparsed_file);

    println!("{:#?}", properties);
}
