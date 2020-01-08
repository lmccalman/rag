use anyhow::Result;
use std::vec::Vec;
use std::collections::HashMap;
use std::string::String;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

use rag::Config;
use rag::state::GameState;
use rag::map;
use rag::parser;
use rag::command;
use rage::net;


// actix actor framework
// https://github.com/actix/examples
// https://medium.com/@fdeantoni/actix-websockets-with-protobuf-bc037a999d89
// https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

// look at mio or tokio for event driven
//
// mioco or futures based such as tokio or futures-rs
//
// look at lua or dyon for writing the actual game content or scripting

fn main() -> Result<()> {

    // initialise game state
    let config = Config::new()?;
    println!("{:?}", config);

    let exmap = map::example();
    map::dump(&exmap, config.outfile)?;
    let loaded_map = map::load(config.infile)?;
    assert_eq!(exmap, loaded_map);

    let mut state = GameState::load(&loaded_map)?;

    let mut running = true;

    let mut cmd = command::Command::System(command::System::Initialise);


    let tick = Duration::from_millis(2000);
    // initialise the server connection thread
    let client_mutex : ClientDB = Arc::new(Mutex::new(HashMap::new()));
    let t_clients = client_mutex.clone();
    thread::spawn(move || { connection_thread(t_clients) });


    // TODO make a server console
    // cmd = parser::parse_input();
    // if let command::Command::System(command::System::Quit) = cmd {
    //     running = false;
    // }

    let mut messages: Vec<(UserID, String)> = Vec::new();
    loop {
        let start = Instant::now();
        // get all the messages we're going to process this tick
        pull_messages(client_mutex.clone(), &start, &mut messages);

        // game loop!!
        for (uid, msg) in messages.iter() {
            println!("Tick message .. id: {} \t msg: {}",uid, msg);
        }

        messages.clear();

        // finish up the tick
        let finish = Instant::now();
        
        let actual_tick_length = finish - start;
        if actual_tick_length > tick {
            println!("Error: tick took too long")
        }
        else
        {
            thread::sleep(tick - actual_tick_length);
        }
    }
    return Ok(());
}
