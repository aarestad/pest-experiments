use std::collections::HashMap;

use pest::error::{Error, ErrorVariant};
use pest::Parser;
use pest_derive::Parser;

type ParsedINI<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;

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
pub fn parse_to_map(input: &str) -> Result<ParsedINI, Error<Rule>> {
    let file = INIParser::parse(Rule::file, input)?.next().expect("bad parsing output");

    let mut properties: ParsedINI = HashMap::default();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.clone().into_inner(); // { name }
                current_section_name = inner_rules.next().expect("bad parsing output").as_str();

                if properties.contains_key(current_section_name) {
                    return Err(Error::new_from_span(
                        ErrorVariant::CustomError{ message: String::from("duplicate section name") },
                        line.as_span()),
                    );
                }

                properties.insert(current_section_name, HashMap::default());
            }
            Rule::property => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let name: &str = inner_rules.next().expect("bad parsing output").as_str();
                let value: &str = inner_rules.next().expect("bad parsing output").as_str();

                if current_section_name.is_empty() && !properties.contains_key("") {
                    properties.insert("", HashMap::default());
                }

                let section = properties
                    .get_mut(current_section_name)
                    .expect("unexpected section name");

                section.insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(properties)
}
