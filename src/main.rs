use anyhow::Result;
use std::vec::Vec;
use std::collections::HashMap;
use std::string::String;
use std::time::{Duration, Instant};
use std::thread;

use rag::UserID;
use rag::Config;
use rag::state::GameState;
use rag::map;
use rag::parser;
use rag::command;
use rag::net;

use log::{info};

// actix actor framework
// https://github.com/actix/examples
// https://medium.com/@fdeantoni/actix-websockets-with-protobuf-bc037a999d89
// https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

// look at mio or tokio for event driven
//
// mioco or futures based such as tokio or futures-rs
//
// look at lua or dyon for writing the actual game content or scripting

fn finish_tick(tick: &Duration, start: &Instant) {
    let finish = Instant::now();

    let actual_tick_length = finish - *start;
    if actual_tick_length > *tick {
        println!("Error: tick took too long")
    }
    else
    {
        thread::sleep(*tick - actual_tick_length);
    }
}

fn main() -> Result<()> {
    
    // Initialise logging
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed).expect("No interactive terminal");

    // initialise game state
    info!("Initialising game state");
    let config = Config::new()?;
    println!("{:?}", config);

    let exmap = map::example();
    map::dump(&exmap, config.outfile)?;
    let loaded_map = map::load(config.infile)?;
    assert_eq!(exmap, loaded_map);

    let mut state = GameState::load(&loaded_map)?;
    let mut cmd = command::Command::System(command::System::Initialise);


    let tick = Duration::from_millis(2000);
    // initialise the server connection thread

    let mut clients = net::ClientInterface::new();
    // fn get_update(&mut self, start_time: &Instant, messages: &mut Vec<(UserID, String)>) {

    // TODO make a server console
    // cmd = 
    // if let command::Command::System(command::System::Quit) = cmd {
    //     running = false;
    // }

    let mut running = true;
    let mut incoming_messages: Vec<(UserID, String)> = Vec::new();
    let mut replies: Vec<(UserID, String)> = Vec::new();
    while running {
        let start = Instant::now();
        println!("((( ---");
        // get all the messages we're going to process this tick
        // pull_messages(client_mutex.clone(), &start, &mut messages);

        clients.get_update(&start, &mut incoming_messages);

        // game loop!!
        let commands = parser::parse_input(&incoming_messages);

        for (uid, msg) in commands.iter() {
            println!(". id: {} \t msg: {:?}",uid, msg);
            replies.push((*uid, "Thanks for your message\r\n> ".to_string()));
        }
        clients.send(&replies)?;

        // finish up the tick
        incoming_messages.clear();
        replies.clear();
        finish_tick(&tick, &start);

        println!("--- )))");
    }
    return Ok(());
}
