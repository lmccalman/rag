use super::{Direction, UserID};
use super::state::GameState;

#[derive(Debug)]
pub enum System {
    Quit,
    Initialise
}

#[derive(Debug)]
pub enum Command {
    System(System),
    Movement(Direction),
    Unknown(String)
}

pub struct UserCommand {
    pub uid: UserID,
    pub cmd: Command,
}

fn movement(commands: &Vec<UserCommand>, s: &mut GameState) {
    for c in commands.iter() {
        // if let Command::Movement(d) = c.cmd {
            
            // let loc = &s.player.location;
            // let room_portals = &s.faceted[loc];
            // if room_portals.contains_key(d)  {
            //     let portal_id = room_portals[d];
            //     let dest = s.portals[&portal_id].to;
            //     // move the player!
            //     s.player.location = dest;
            // }
        // }
    }
}

pub fn process(c: &Vec<UserCommand>, s: &mut GameState) {

    movement(c, s);
    // this haapens LAST
    // render(c, s);

}
