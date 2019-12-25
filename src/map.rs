use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct EntityID(i64);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    InRoom { id: EntityID },
    InInventory { id: EntityID }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Component {
    Renderable { name: String, short: String, long: Option<String>},
    Room { z: i64 },
    Object { location: Location },
    Edible,
    Player,
    Portal { from: EntityID, to: EntityID }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityID,
    pub name: String,
    pub comps: Vec<Component>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub entities: HashMap<EntityID, Entity>
}

pub struct IDGenerator(std::ops::Range<i64>);

pub fn example() -> Map {

    let gen = std::ops::Range {start: 0, end: 2^32};
    let mut exmap = Map {name: "Test Map".to_string(), entities: HashMap::new()};

    let room1id = EntityID(1);
    let mut room1 = Box::new(Entity {id: room1id, name: "room1".to_string(), comps: Vec::new()}); 
    room1.comps.push(Component::Renderable {
                name: "Lachy's Room".to_string(),
                short: "The room is dark".to_string(),
                long: Some("This is a crazy dark room".to_string())
            });
    room1.comps.push(Component::Room {z: 1});
    exmap.entities.insert(room1id, *room1);

    let room2id = EntityID(2);
    let mut room2 = Entity {id: room2id, name: "room2".to_string(), comps: Vec::new()};
    room2.comps.push(Component::Renderable {
                name: "Andrew's Room".to_string(),
                short: "The room is bright".to_string(),
                long: None
            });
    room2.comps.push(Component::Room {z: 1});
    exmap.entities.insert(room2id, room2);
    
    let door1id = EntityID(3);
    let mut door1 = Entity {id: door1id, name: "door1".to_string(), comps: Vec:: new()};
    door1.comps.push(Component::Renderable {
                name: "Big Door".to_string(),
                short: "A large wooden door".to_string(),
                long: None
            });
    door1.comps.push(Component::Object {location: Location::InRoom { id: room1id.clone() }});
    door1.comps.push(Component::Portal { from: room1id.clone(), to: room2id.clone() });
    exmap.entities.insert(door1id, door1);
    
    let player1id = EntityID(4);
    let mut player1 = Entity {id: player1id, name:"player1".to_string(), comps: Vec::new()};
    player1.comps.push(Component::Renderable {
                name: "Joe the Mage".to_string(),
                short: "A small, shaggy mage".to_string(),
                long: None
            });
    player1.comps.push(Component::Player);
    exmap.entities.insert(player1id, player1);
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

