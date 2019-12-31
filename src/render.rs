use console::Term;   
use super::command::{Command, System};
use super::state::GameState;

pub fn render(c: &Command, s: &mut GameState, t: &Term) {

    match c {
        Command::System(System::Initialise) => {
            //print the description of the players location
            let short = &s.shorts[&s.player.location];
            t.write_line(short).unwrap();
        },
        _ => {}
    }
}

