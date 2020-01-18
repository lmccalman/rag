use anyhow::Result;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude::*;
use crossbeam::channel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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

type ClientMap = HashMap<UserID, Client>;

pub struct ClientInterface{
    db: ClientMap,
    new_users: channel::Receiver<(UserID, Client)>,
    handle: thread::JoinHandle<Result<()>>,
}


impl ClientInterface {

    pub fn new() -> ClientInterface {
        let db = HashMap::new();
        let (new_in, new_out) = channel::unbounded();
        let t_new_in = new_in.clone();
        let handle = thread::spawn(move || { connection_thread(t_new_in) });
        return ClientInterface {db: db, new_users: new_out, handle: handle};
    }

    pub fn send(&mut self, messages: &Vec<(UserID, String)>) -> Result<()> {
        for (uid, s) in messages.iter() {
            if let Some(c) = self.db.get(uid) {
                c.snd_in.send(CommsMsg { time: Instant::now(), msg: s.clone() })?;
            }
        }
        return Ok(());
    }

    pub fn get_update(&mut self, start_time: &Instant, messages: &mut Vec<(UserID, String)>) {
        let v: Vec<_> = self.new_users.try_iter().collect();
        for (uid, client) in v {
            self.db.insert(uid, client);
            info!("Added new client");
        }

        for (uid, s) in self.db.iter() {
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

fn login_thread(sender: channel::Sender<(UserID, Client)>,
                mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut login = String::new();
    let mut password = String::new();

    stream.write("Username: ".as_bytes())?;
    reader.read_line(&mut login)?;
    stream.write("Password: ".as_bytes())?;
    reader.read_line(&mut password)?;
    let uid = calculate_hash(login);
    sender.send((uid, Client::new(stream).unwrap()))?;
    return Ok(());
}


fn connection_thread(sender: channel::Sender<(UserID, Client)>) -> Result<()> {

    let listener = TcpListener::bind("0.0.0.0:3334")?;
    println!("Server listening on port 3334");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let thread_sender = sender.clone();
                thread::spawn(move || { login_thread(thread_sender, stream) });
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

