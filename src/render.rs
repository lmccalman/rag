use super::command::Command;
use anyhow::Result;
use super::state::GameState;

pub fn render(c: &Command, s: &mut GameState) -> Result<()> {

    if let Command::Unknown(s) = c {
        println!("Unknown command: {}", s);
    }

    //print the description of the players location
    let playerloc = s.player.location;
    if let Some(short) = s.shorts.get(&playerloc) {
        println!("{}", short);
    }

    // list the portals in the room
    if let Some(facets) = s.faceted.get(&playerloc) {
        for (k, _) in facets {
            println!("There is a door to the {}", k);
        }
    }

    if let Some(inroom) = s.containers.get(&playerloc) {
        if inroom.len() > 0 {
            println!("In the room there is");
            for o in inroom {
                let short = &s.shorts[o];
                println!("- a {}", short);
            }
        }
    }
    return Ok(());
}

