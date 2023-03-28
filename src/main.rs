use std::collections::HashMap;
use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " }

char = { ASCII_ALPHANUMERIC | "." | "_" | "/" }

name = @{ char+ }
value = @{ char* }

section = { "[" ~ name ~ "]" }
property = { name ~ "=" ~ value }

file = {
  SOI ~
  ((section | property)? ~ NEWLINE)* ~
  EOI
}
"#]
pub struct INIParser;

fn main() {
    let unparsed_file = fs::read_to_string("config.ini").expect("cannot read file");

    let file = INIParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::default();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner(); // { name }
                current_section_name = inner_rules.next().unwrap().as_str();

                if properties.contains_key(current_section_name) {
                    panic!("duplicate section name");
                }

                properties.insert(current_section_name, HashMap::default());
            }
            Rule::property => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let name: &str = inner_rules.next().unwrap().as_str();
                let value: &str = inner_rules.next().unwrap().as_str();

                if current_section_name == "" && !properties.contains_key("") {
                    properties.insert("", HashMap::default());
                }
                
                let section = properties.get_mut(current_section_name).expect("section not found");

                section.insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}
