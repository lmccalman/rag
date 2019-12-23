use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityID(i64);
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomID(i64);
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectID(i64);
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryID(i64);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Location {
    InRoom { id: RoomID },
    InInventory { id: InventoryID }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Component {
    Renderable { name: String, short: String, long: Option<String>},
    Room { id: RoomID, z: i64 },
    Object { location: Location},
    Edible,
    Player,
    Portal { from: RoomID, to: RoomID }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityID,
    pub comps: Vec<Component>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub entities: Vec<Entity>
}

pub fn example() -> Map {

    let room1 = Entity {
        id: EntityID(1),
        comps: vec![
            Component::Renderable {
                name: "Lachy's Room".to_string(),
                short: "The room is dark".to_string(),
                long: Some("This is a crazy dark room".to_string())
            },
            Component::Room {id: RoomID(1), z: 1}
        ]
    };

    let room2 = Entity {
        id: EntityID(2),
        comps: vec![
            Component::Renderable {
                name: "Andrew's Room".to_string(),
                short: "The room is bright".to_string(),
                long: None
            },
            Component::Room {id: RoomID(2), z: 1}
        ]
    };

    let door1 = Entity {
        id: EntityID(3),
        comps: vec![
            Component::Renderable {
                name: "Big Door".to_string(),
                short: "A large wooden door".to_string(),
                long: None
            },
            Component::Object {
                location: Location::InRoom { id: RoomID(2) }
            },
            Component::Portal {
                from: RoomID(1),
                to: RoomID(2)
            }
        ]
    };
    
    let player1 = Entity {
        id: EntityID(3),
        comps: vec![
            Component::Renderable {
                name: "Joe the Mage".to_string(),
                short: "A small, shaggy mage".to_string(),
                long: None
            },
            Component::Player
        ]
    };
    
    let exmap = Map {name: "Test Map".to_string(), entities: vec![room1, room2, door1, player1] };
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

