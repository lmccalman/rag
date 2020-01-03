use anyhow::Result;
use rag::Config;
use rag::state::GameState;
use rag::map;
use rag::render;
use rag::parser;
use rag::command;
use console::Term;   
use console::style;
use dialoguer::{theme::CustomPromptCharacterTheme, Input};


fn main() -> Result<()> {
    
    let cursortheme = CustomPromptCharacterTheme::new('>');
    
    let config = Config::new()?;
    println!("{:?}", config);

    let exmap = map::example();
    map::dump(&exmap, config.outfile)?;
    let loaded_map = map::load(config.infile)?;
    assert_eq!(exmap, loaded_map);

    let mut state = GameState::load(&loaded_map)?;

    let mut running = true;
    let term = Term::stdout();
    term.clear_screen()?;
    term.write_line(&format!("Welcome to {}", style(&exmap.name).cyan()))?;

    let mut cmd = command::Command::System(command::System::Initialise);

    while running {

        command::process(&cmd, &mut state, &term);
        
        let buffer: String = Input::with_theme(&cursortheme).with_prompt("").interact()?;
        cmd = parser::parse_input(buffer.trim());
        if let command::Command::System(command::System::Quit) = cmd {
            running = false;
        }
    }

    println!("Goodbye");
    Ok(())
}
