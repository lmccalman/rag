use pest::Parser;
use super::command::{Command, System};
use super::state::Direction;
use std::io::{self, Read};

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;


pub fn parse_input(s: &str) -> Command {

    // todo -- do the wor in a fn with result so I can use ? then wrap at end for command
    let mut s = String::new();
    let j = io::stdin().read_to_string(&mut s);
    if let Ok(c) = &mut InputParser::parse(Rule::command, &s) {
        
        // never fails apparently
        let t = c.next().unwrap(); 

        for i in t.into_inner() {
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
    else {
        return Command::Unknown(s.to_string());
    }
}
