use super::state::{GameState, Direction};
use super::render::render;


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

fn movement(c: &Command, s: &mut GameState){

    if let Command::Movement(d) = c {

        let loc = &s.player.location;
        let room_portals = &s.faceted[loc];
        if room_portals.contains_key(d)  {
            let portal_id = room_portals[d];
            let dest = s.portals[&portal_id].to;
            // move the player!
            s.player.location = dest;
        }
    }
}

pub fn process(c: &Command, s: &mut GameState) {

    movement(c, s);
    // this haapens LAST
    render(c, s);

}
