use anyhow::Result;
use rag::Config;
use rag::state::GameState;
use rag::map;
use console::Term;   
use console::style;
use dialoguer::{theme::CustomPromptCharacterTheme, Input};

//fn render_map(map:  &Map,  t: &Term) -> Result<()> {

//    let player_loc : EntityID;
//    // = EntityID(-1);
//    match map.player.location {
//        Location::InRoom(id) => player_loc = id,
//        Location::InInventory(_) => return Ok(()) 
//    }
//    dbg!("Player is in room {}", player_loc);

//    for e in &map.entities {
//        if let Some(ref r) = e.renderable {
//            println!("{}", r.short)
//        }
//    }

//    //locate player
//    return Ok(());

//}

fn main() -> Result<()> {
    
    let cursortheme = CustomPromptCharacterTheme::new('>');
    
    let config = Config::new()?;
    println!("{:?}", config);

    let exmap = map::example();
    map::dump(&exmap, config.outfile)?;

    let mut running = true;
    let term = Term::stdout();
    term.clear_screen()?;
    term.write_line(&format!("Welcome to {}", style(&exmap.name).cyan()))?;

    let state = GameState::load(&exmap);

    dbg!(state);

    while running {

        // render_map(&exmap, &term)?;

        
        let buffer: String = Input::with_theme(&cursortheme).with_prompt("").interact()?;
        if buffer.trim() == "!q" {
            println!("Goodbye");
            running = false;
        }
    }

    // read the result back

    let loaded_map = map::load(config.infile)?;
    assert_eq!(exmap, loaded_map);
    println!("{:?}", loaded_map);
    Ok(())
}
