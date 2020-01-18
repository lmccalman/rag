use pest::Parser;
use super::command::{Command, System, UserCommand};
use super::Direction;
// use std::io::{self, BufRead};
use super::UserID;

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;


fn parse_string(s: &String) -> Command {
    let t = s.trim().clone();
    if let Ok(c) = &mut InputParser::parse(Rule::command, &t) {
        
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

pub fn parse_input(msgs: &Vec<(UserID, String)>) -> Vec<UserCommand> {

    let mut cmds : Vec<UserCommand> = Vec::new();
    for (uid, s) in msgs.iter() {
        cmds.push(UserCommand {uid: *uid, cmd: parse_string(s)} );
    }
    return cmds;
    // todo -- do the wor in a fn with result so I can use ? then wrap at end for command
    // let mut s = String::new();
    // let stdin = io::stdin();
    // stdin.lock().read_line(&mut s).unwrap();
    // println!("I got input: {}", s);


}
