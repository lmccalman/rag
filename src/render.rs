use console::Term;   
use super::command::{Command, System};
use super::state::GameState;

pub fn render(c: &Command, s: &mut GameState, t: &Term) {

    //print the description of the players location
    let playerloc = s.player.location;
    let short = &s.shorts[&playerloc];
    t.write_line(short).unwrap();

    // list the portals in the room
    for (k, _) in &s.faceted[&playerloc] {
        println!("There is a door to the {}", k);
    }

    let inroom = &(s.containers[&playerloc]);

    if inroom.len() > 0 {
        println!("In the room there is");
        for o in inroom {
            let short = &s.shorts[o];
            println!("- a {}", short);
        }
    }
}

