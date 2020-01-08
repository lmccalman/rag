use anyhow::Result;
use std::vec::Vec;
use std::collections::HashMap;
use std::string::String;
use std::thread;
use std::time::{Duration, Instant};
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude::*;
use crossbeam::channel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

type UserID = u64;
type ClientDB = Arc<Mutex<HashMap<UserID, Client>>>;
struct CommsMsg {
    time: Instant,
    msg: String
}

#[derive(Hash)]
struct StringHash(String);

fn calculate_hash(mystring: String) -> UserID {
    let o = StringHash(mystring);
    let mut s = DefaultHasher::new();
    o.hash(&mut s);
    s.finish()
}

fn sender_thread(mut s: TcpStream, rcv: channel::Receiver<CommsMsg>) -> Result<()> {
    loop {
        if let Ok(m) = rcv.recv() {
            s.write(m.msg.as_bytes())?;
        }
    }
}

fn receiver_thread(s: TcpStream, snd: channel::Sender<CommsMsg>) -> Result<()> {
    let mut reader = BufReader::new(s);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let cm = CommsMsg {time: Instant::now(), msg: line};
        snd.send(cm)?;
    }
}

pub struct Client {
    snd: channel::Sender<CommsMsg>,
    rcv: channel::Receiver<CommsMsg>,
    sender_handle: thread::JoinHandle<Result<()>>,
    receiver_handle: thread::JoinHandle<Result<()>>,
}

impl Client {
    fn new(stream: TcpStream) -> Result<Client> {
        // let (snd, rcv) : (channel::Sender<CommsMsg>, channel::Receiver<CommsMsg>) = channel::unbounded();
        let (snd, rcv) = channel::unbounded();
        let t_snd = snd.clone();
        let t_rcv = rcv.clone();
        let stream_snd = stream.try_clone()?;
        let stream_rcv = stream.try_clone()?;
        let sender_handle = thread::spawn(move|| {sender_thread(stream_snd, t_rcv)});
        let receiver_handle = thread::spawn(move|| {receiver_thread(stream_rcv, t_snd)});
        return Ok(Client {snd, rcv, sender_handle, receiver_handle});
    }
}

fn login_thread(clients: ClientDB, mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut login = String::new();
    let mut password = String::new();
    stream.write("Username: ".as_bytes())?;
    reader.read_line(&mut login)?;
    stream.write("Password: ".as_bytes())?;
    reader.read_line(&mut password)?;
    let uid = calculate_hash(login);
    // insert this into the client DB
    {
        let mut map = clients.lock().unwrap();
        map.insert(uid, Client::new(stream).unwrap());
    }
    return Ok(());
}


fn connection_thread(db: ClientDB) -> Result<()> {

    let listener = TcpListener::bind("0.0.0.0:3333")?;
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let thread_db = db.clone();
                thread::spawn(move || { login_thread(thread_db, stream) });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
    return Ok(());
}


fn pull_messages(db: ClientDB, start_time: &Instant, messages: &mut Vec<(UserID, String)>) {
    let clients = db.lock().unwrap();
    for (uid, s) in clients.iter() {
        let mut it = s.rcv.try_iter().peekable();
        loop {
            match it.peek() {
                Some(m) => { 
                    if m.time < *start_time {
                        if let Some(a) = it.next() {
                            messages.push((uid.clone(), a.msg));
                        }
                    } 
                    else {
                        break;
                    }
                },
                None => {break}
            }
        }
    }
}

fn main() -> Result<()> {
    let tick = Duration::from_millis(500);
    // initialise the server connection thread
    let client_mutex : ClientDB = Arc::new(Mutex::new(HashMap::new()));
    let t_clients = client_mutex.clone();
    thread::spawn(move || { connection_thread(t_clients) });

    let mut messages: Vec<(UserID, String)> = Vec::new();
    loop {
        let start = Instant::now();
        // get all the messages we're going to process this tick
        pull_messages(client_mutex.clone(), &start, &mut messages);

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
