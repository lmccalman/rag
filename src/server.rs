#![warn(rust_2018_idioms)]

use anyhow::Result;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tokio::runtime::Builder;
use tokio::prelude::*;
use tokio::runtime::Runtime;

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

/// The in-memory database shared amongst all clients.
///
/// This database will be shared via `Arc`, so to mutate the internal map we're
/// going to use a `Mutex` for interior mutability.
struct Database {
    map: Mutex<HashMap<String, String>>,
}

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


fn tcpserver () -> Result<()>{

    let mut rt = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime

    let child = thread::spawn(move || { tcpserver() });

    loop {


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


//     rt.block_on( async {

//     let addr = "127.0.0.1:8080".to_string();
//     let mut listener = TcpListener::bind(&addr).await.unwrap();
//     println!("Listening on: {}", addr);

//     // Create the shared state of this server that will be shared amongst all
//     // clients. We populate the initial database and then create the `Database`
//     // structure. Note the usage of `Arc` here which will be used to ensure that
//     // each independently spawned client will have a reference to the in-memory
//     // database.
//     let mut initial_db = HashMap::new();
//     initial_db.insert("foo".to_string(), "bar".to_string());
//     let db = Arc::new(Database {
//         map: Mutex::new(initial_db),
//     });

//     loop {
//         match listener.accept().await {
//             Ok((socket, _)) => {
//                 // After getting a new connection first we see a clone of the database
//                 // being created, which is creating a new reference for this connected
//                 // client to use.
//                 let db = db.clone();

//                 // Like with other small servers, we'll `spawn` this client to ensure it
//                 // runs concurrently with all other clients. The `move` keyword is used
//                 // here to move ownership of ocontainersur db handle into the async closure.
//                 tokio::spawn(async move {
//                     // Since our protocol is line-based we use `tokio_codecs`'s `LineCodec`
//                     // to convert our stream of bytes, `socket`, into a `Stream` of lines
//                     // as well as convert our line based responses into a stream of bytes.
//                     let mut lines = Framed::new(socket, LinesCodec::new());

//                     // Here for every line we get back from the `Framed` decoder,
//                     // we parse the request, and if it's valid we generate a response
//                     // based on the values in the database.
//                     while let Some(result) = lines.next().await {
//                         match result {
//                             Ok(line) => {
//                                 let response = handle_request(&line, &db);

//                                 let response = response.serialize();

//                                 if let Err(e) = lines.send(response).await {
//                                     println!("error on sending response; error = {:?}", e);
//                                 }
//                             }
//                             Err(e) => {
//                                 println!("error on decoding from socket; error = {:?}", e);
//                             }
//                         }
//                     }

//                     // The connection will be closed at this point as `lines.next()` has returned `None`.
//                 });
//             }
//             Err(e) => println!("error accepting socket; error = {:?}", e),
//         }
//     }
//     });
// }

fn handle_request(line: &str, db: &Arc<Database>) -> Response {
    let request = match Request::parse(&line) {
        Ok(req) => req,
        Err(e) => return Response::Error { msg: e },
    };

    let mut db = db.map.lock().unwrap();
    match request {
        Request::Get { key } => match db.get(&key) {
            Some(value) => Response::Value {
                key,
                value: value.clone(),
            },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = db.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
                previous,
            }
        }
    }
}

impl Request {
    fn parse(input: &str) -> Result<Request, String> {
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("GET must be followed by a key")?;
                if parts.next().is_some() {
                    return Err("GET's key must not be followed by anything".into());
                }
                Ok(Request::Get {
                    key: key.to_string(),
                })
            }
            Some("SET") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("SET must be followed by a key".into()),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("SET needs a value".into()),
                };
                Ok(Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(cmd) => Err(format!("unknown command: {}", cmd)),
            None => Err("empty input".into()),
        }
    }
}

impl Response {
    fn serialize(&self) -> String {
        match *self {
            Response::Value { ref key, ref value } => format!("{} = {}", key, value),
            Response::Set {
                ref key,
                ref value,
                ref previous,
            } => format!("set {} = `{}`, previous: {:?}", key, value, previous),
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}
