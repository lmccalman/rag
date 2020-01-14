use anyhow::Result;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude::*;
use crossbeam::channel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::time::Instant;

pub type UserID = u64;

// http://danzig.jct.ac.il/tcp-ip-lab/ibm-tutorial/3376c42.html
// http://mud-dev.wikidot.com/telnet:negotiation

enum TelnetCommand {
    SE = 240,
    NOP = 241,
    DataMark = 242,
    Break = 243,
    GoAhead = 249,
    SB = 250,
    WILL = 251,
    WONT = 252,
    DO = 253,
    DONT = 254,
    IAC = 255
}

type ClientMap = Arc<Mutex<HashMap<UserID, Client>>>;

pub struct ClientInterface{
    db: ClientMap,
    handle: thread::JoinHandle<Result<()>>,
}


impl ClientInterface {

    pub fn new() -> ClientInterface {
        let db = Arc::new(Mutex::new(HashMap::new()));
        let t_db = db.clone();
        let handle = thread::spawn(move || { connection_thread(t_db) });
        return ClientInterface {db, handle};
    }

    pub fn send(&mut self, messages: &Vec<(UserID, String)>) -> Result<()> {
        let db = self.db.lock().unwrap();
        for (uid, s) in messages.iter() {
            if let Some(c) = db.get(uid) {
                c.snd_in.send(CommsMsg { time: Instant::now(), msg: s.clone() })?;
            }
        }
        return Ok(());
    }

    pub fn get_update(&mut self, start_time: &Instant, messages: &mut Vec<(UserID, String)>) {
        let clients = self.db.lock().unwrap();
        for (uid, s) in clients.iter() {
            let mut it = s.rcv_out.try_iter().peekable();
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
}

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

struct Client {
    snd_in: channel::Sender<CommsMsg>,
    rcv_out: channel::Receiver<CommsMsg>,
    sender_handle: thread::JoinHandle<Result<()>>,
    receiver_handle: thread::JoinHandle<Result<()>>,
}

impl Client {
    fn new(stream: TcpStream) -> Result<Client> {
        // let (snd, rcv) : (channel::Sender<CommsMsg>, channel::Receiver<CommsMsg>) = channel::unbounded();
        let (snd_in, snd_out) = channel::unbounded();
        let (rcv_in, rcv_out) = channel::unbounded();
        let t_snd = snd_out.clone();
        let t_rcv = rcv_in.clone();
        let stream_snd = stream.try_clone()?;
        let stream_rcv = stream.try_clone()?;
        let sender_handle = thread::spawn(move|| {sender_thread(stream_snd, t_snd)});
        let receiver_handle = thread::spawn(move|| {receiver_thread(stream_rcv, t_rcv)});
        return Ok(Client {snd_in, rcv_out, sender_handle, receiver_handle});
    }
}

fn login_thread(clients: ClientMap, mut stream: TcpStream) -> Result<()> {
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


fn connection_thread(db: ClientMap) -> Result<()> {

    let listener = TcpListener::bind("0.0.0.0:3334")?;
    println!("Server listening on port 3334");
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

