use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityName(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Room {
        pub name: EntityName, 
        pub short: String, 
        pub long: Option<String> 
} 
    
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
        pub name: EntityName,
        pub location: EntityName,
        pub short: String, 
        pub long: Option<String>,
        pub movable: bool
}
    
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Container {
        pub name: EntityName,
        pub location: EntityName, 
        pub short: String,
        pub long: Option<String>,
        pub movable: bool,
        pub capacity: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Portal {
        pub name: EntityName,
        pub location: EntityName, 
        pub short: String,
        pub long: Option<String>,
        pub from: EntityName,
        pub to: EntityName,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Player { 
    pub name: EntityName,
    pub location: EntityName 
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub player: Player,
    pub rooms: Vec<Room>,
    pub objects: Vec<Object>,
    pub containers: Vec<Container>,
    pub portals: Vec<Portal>,
}

pub fn example() -> Map {

    let room1id = EntityName("room1".to_string());
    let mut exmap = Map {
        name: "Test Map".to_string(),
        rooms: Vec::new(),
        objects: Vec::new(),
        containers: Vec::new(),
        portals: Vec::new(),
        player: Player {name: EntityName("player1".to_string()), location: room1id.clone()}
    };

    let room1 = Room {
        name: room1id.clone(),
        short: "The room is dark".to_string(),
        long: Some("This is a crazy dark room".to_string())
    };
    exmap.rooms.push(room1);

    let room2id = EntityName("room2".to_string());
    let room2 = Room {
        name: room2id.clone(),
        short: "The room is bright".to_string(),
        long: None
    };
    exmap.rooms.push(room2);
    
    let door1id = EntityName("door1".to_string());
    let door1 = Portal {
        name: door1id, 
        short: "A large wooden door".to_string(),
        long: None,
        location: room1id.clone(),
        from: room1id.clone(),
        to: room2id.clone(),
    };
    exmap.portals.push(door1);
    
    return exmap
}
    
pub fn dump<P: AsRef<Path>>(m: &Map, filename: P) -> Result<()> {

    let s = serde_yaml::to_string(&m)?;
    let mut file = File::create(filename)?;
    file.write_all(s.as_bytes())?;
    return Ok(());
}
    
pub fn load<P: AsRef<Path>>(filename: P) -> Result<Map> {

    let contents = fs::read_to_string(filename)?;
    let m: Map = serde_yaml::from_str(&contents)?;
    return Ok(m);
}

