use anyhow::Result;
use rag::Config;
use rag::state::GameState;
use rag::map;
use rag::parser;
use rag::command;

// actix actor framework
// https://github.com/actix/examples
// https://medium.com/@fdeantoni/actix-websockets-with-protobuf-bc037a999d89
// https://github.com/actix/actix/blob/master/examples/chat/src/main.rs



fn main() -> Result<()> {
    
    
    let config = Config::new()?;
    println!("{:?}", config);

    let exmap = map::example();
    map::dump(&exmap, config.outfile)?;
    let loaded_map = map::load(config.infile)?;
    assert_eq!(exmap, loaded_map);

    let mut state = GameState::load(&loaded_map)?;

    let mut running = true;

    let mut cmd = command::Command::System(command::System::Initialise);

    while running {

        command::process(&cmd, &mut state);

        cmd = parser::parse_input();
        if let command::Command::System(command::System::Quit) = cmd {
            running = false;
        }
    }

    println!("Goodbye");
    Ok(())
}
