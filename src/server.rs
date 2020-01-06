#![warn(rust_2018_idioms)]

use anyhow::Result;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tokio::runtime::Builder;
use tokio::prelude::*;
use tokio::runtime::Runtime;

use std::sync::{Arc, Mutex};
use futures::SinkExt;
use std::collections::HashMap;
use std::vec::Vec;
use std::string::String;
use std::env;
use std::thread;
use std::time::Duration;

/// Possible requests our clients can send us
enum Request {
    Get { key: String },
    Set { key: String, value: String },
}

/// Responses to the `Request` commands above
enum Response {
    Value {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        previous: Option<String>,
    },
    Error {
        msg: String,
    },
}

fn tcpserver(q: Arc<Mutex<Vec<String>>>) {

    let mut rt = Runtime::new().unwrap();
    rt.block_on(
        async {
            let addr = "127.0.0.1:8080".to_string();
            let mut listener = TcpListener::bind(&addr).await.unwrap();
            println!("Listening on: {}", addr);
            loop {
                let (socket, _) = listener.accept().await.unwrap();
                let localqueue = q.clone();
                tokio::spawn(
                    async move {
                        let mut lines = Framed::new(socket, LinesCodec::new());
                        while let Some(result) = lines.next().await {
                            match result {
                                Ok(line) => {
                                    {
                                        let mut mq = localqueue.lock().unwrap();
                                        mq.push(line);
                                    }
                                    let response = String::from("Your input is noted.");
                                    lines.send(response).await.unwrap();
                                }
                                Err(e) => {
                                    println!("error on decoding from socket; error = {:?}", e);
                                }
                            } // end match result
                        } // end line iteration
                    // connection closed as `lines.next()` returned `None`.
                    } // end async move block
                ); // end spawn
            } // end loop
        } // end async
    ); // end block_on
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime

    let mut queue : Vec<String> = Vec::new();
    let aqueue = Arc::new( Mutex::new(queue) );
    let tqueue = aqueue.clone();
    let child = thread::spawn(move || { tcpserver(tqueue) });

    let tick = Duration::from_millis(1000);
    loop {
        thread::sleep(tick);
        {
            println!("staring lock");
            {
                let myqueue = aqueue.lock().unwrap();
                println!("{}", myqueue.len());
            }
            println!("ending lock");
        }

    }

    return Ok(());
}


// fn main() {
//     // build runtime
//     let mut rt = Builder::new()
//         .threaded_scheduler()
//         .core_threads(4)
//         .thread_name("my-custom-name")
//         .thread_stack_size(3 * 1024 * 1024)
//         .build()
//         .unwrap();

