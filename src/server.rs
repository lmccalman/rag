use anyhow::Result;
use std::vec::Vec;
use std::collections::HashMap;
use std::string::String;
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use crossbeam::channel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type UserID = u64;

pub struct ClientIO {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>
}

impl ClientIO {

    fn new(s: TcpStream) -> Result<ClientIO> {

        let mut ws = s.try_clone()?;
        let mut reader = BufReader::new(s);
        let mut writer = BufWriter::new(ws);
        return Ok(ClientIO {reader, writer})
    }
    
    fn read_line(&mut self) -> Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        return Ok(line)
    }

    fn write_lines(&mut self, msgs: &Vec<String>) -> Result<()> {
        for m in msgs {
            self.writer.write(m.as_bytes())?;
        }
        self.writer.flush()?;
        return Ok(());
    }
}

#[derive(Hash)]
struct StringHash(String);

fn calculate_hash(mystring: String) -> u64 {
    let o = StringHash(mystring);
    let mut s = DefaultHasher::new();
    o.hash(&mut s);
    s.finish()
}

pub struct Comms {
    clients: HashMap<UserID, ClientIO>
}

fn comms_thread(s: TcpStream, snd: channel::Sender<String>, 
                rcv: channel::Receiver<String>) -> Result<()> {
    let io = ClientIO::new(s);
    
    loop {
        // check for new messages from stream
        // check for new messages from rcv
        // forward them


    }

    return Ok(());
}

struct Client {
    snd: channel::Sender<String>,
    rcv: channel::Receiver<String>,
    sender_handle: thread::JoinHandle<Result<()>>,
    receiver_handle: thread::JoinHandle<Result<()>>
    stream: TcpStream
}

impl Client {
    fn new(stream: TcpStream) -> Client {
        let (snd, rcv) = channel::unbounded();
        let t_snd = snd.clone();
        let t_rcv = rcv.clone();
        let sender_handle = thread::spawn(move|| {sender_thread(s, t_rcv)});
        let receiver_handle = thread::spawn(move|| {receiver_thread(s, t_snd)});
        return Client {snd, rcv, sender_handle, receiver_handle, stream}
    }
}



fn tcpserver() -> Result<()> {

    let listener = TcpListener::bind("0.0.0.0:3333")?;
    println!("Server listening on port 3333");
    let mut pre_client: Vec<Client> = Vec::new();
    let mut clients: HashMap<u64, Client> = HashMap::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                pre_client.push(Client::new(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let mut i = 0;
        while i != pre_client.len() {
    
            let e = &mut pre_client[i]; 
            if let Ok(s) = e.rcv.try_recv() {
                let uid = calculate_hash(s); 
                let mye = pre_client.remove(i);
                clients.insert(uid, mye);
            }
            else {
                i += 1;
            }
        }
    }
    // close the socket server
    drop(listener);
    return Ok(());
}

fn main() -> Result<()> {
    // Create the runtime
    let child = thread::spawn(move || { tcpserver() });
    let tick = Duration::from_millis(1000);
    loop {
        thread::sleep(tick);
    }
    return Ok(());
}
