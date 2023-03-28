use std::collections::HashMap;

use pest::error::Error;
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
struct INIParser;

#[allow(clippy::result_large_err)]
pub fn parse_to_map(input: &str) -> Result<HashMap<&str, HashMap<&str, &str>>, Error<Rule>> {
    let file = match INIParser::parse(Rule::file, input) {
        Ok(mut parsed) => parsed.next().unwrap(),
        Err(e) => return Err(e),
    };

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

                if current_section_name.is_empty() && !properties.contains_key("") {
                    properties.insert("", HashMap::default());
                }

                let section = properties
                    .get_mut(current_section_name)
                    .expect("section not found - bug!"); // shouldn't happen

                section.insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(properties)
}
