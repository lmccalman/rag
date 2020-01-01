use pest::Parser;
use super::command::{Command, System};
use super::state::Direction;

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;



pub fn parse_input(s: &str) -> Command {

    let c = InputParser::parse(Rule::command, s).expect("Parse Error").next().unwrap(); 

    for i in c.into_inner() {
        match i.as_rule() {
            Rule::shortcut => { 
                match i.as_str() {
                    "n" => return Command::Movement(Direction::North),
                    "ne" => return Command::Movement(Direction::Northeast),
                    "e" => return Command::Movement(Direction::East),
                    "se" => return Command::Movement(Direction::Southeast),
                    "s" => return Command::Movement(Direction::South),
                    "sw" => return Command::Movement(Direction::Southwest),
                    "w" => return Command::Movement(Direction::West),
                    "nw" => return Command::Movement(Direction::Northwest),
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
