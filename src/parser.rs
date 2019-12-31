use pest::Parser;
use super::command::{Command, Movement, System};

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;



pub fn parse_input(s: &str) -> Command {

    let c = InputParser::parse(Rule::command, s).expect("Parse Error").next().unwrap(); 

    for i in c.into_inner() {
        match i.as_rule() {
            Rule::shortcut => { 
                match i.as_str() {
                    "n" => return Command::Movement(Movement::North),
                    "s" => return Command::Movement(Movement::South),
                    "e" => return Command::Movement(Movement::East),
                    "w" => return Command::Movement(Movement::West),
                    _ => {},
                };
            },
            Rule::system => { 
                for s in i.into_inner() {
                    match s.as_str() {
                        "q" => return Command::System(System::Quit),
                        _ => {},
                    }
                }
            },
            Rule::EOI => {},
            _ => {} 
        }
    }
    return Command::Unknown(s.to_string());
}
