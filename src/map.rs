use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use anyhow::Result;
use super::Name;
use super::Direction;
use serde::{Serialize, Deserialize};
use super::maptraits::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Room {
        pub name: Name, 
        pub short: String, 
        pub long: String,
        pub contains: Vec<Name>,
        pub capacity: usize,
        pub walls: Vec<(Name, Direction)>
} 

impl HasRender for Room {
    fn get(&self) -> RenderInfo {
        return RenderInfo {short: &self.short, long: &self.long};
    }
}
impl HasStorageChest for Room {
    fn get(&self) -> StorageChestInfo {
        let vec = self.contains.iter().map(|x| &x[..]).collect();
        return StorageChestInfo { contains: vec, capacity: self.capacity.clone() }
    }
}
impl HasStorageFacet for Room {
    fn get(&self) -> StorageFacetInfo {
        let vec = self.walls.iter().map(|x| (&x.0[..], x.1.clone())).collect();
        return StorageFacetInfo{ contains: vec };

    }
}
impl HasStorageCharacter for Room {
    fn get(&self) -> StorageCharacterInfo {
        return StorageCharacterInfo;
    }
}

    
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
        pub name: Name,
        pub short: String, 
        pub long: String,
        pub movable: bool
}
impl HasRender for Object {
    fn get(&self) -> RenderInfo {
        return RenderInfo {short: &self.short, long: &self.long};
    }
}
impl HasMovable for Object {
    fn get(&self) -> MovableInfo {
        return MovableInfo {movable: self.movable};
    }
}
    
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Container {
        pub name: Name,
        pub short: String,
        pub long: String,
        pub movable: bool,
        pub capacity: usize,
        pub contains: Vec<Name>,
}

impl HasRender for Container {
    fn get(&self) -> RenderInfo {
        return RenderInfo {short: &self.short, long: &self.long};
    }
}
impl HasStorageChest for Container {
    fn get(&self) -> StorageChestInfo {
        let vec = self.contains.iter().map(|x| &x[..]).collect();
        return StorageChestInfo { contains: vec, capacity: self.capacity.clone() }
    }
}
impl HasMovable for Container {
    fn get(&self) -> MovableInfo {
        return MovableInfo {movable: self.movable};
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Portal {
        pub name: Name,
        pub short: String,
        pub long: String,
        pub to: Name,
}
impl HasRender for Portal {
    fn get(&self) -> RenderInfo {
        return RenderInfo {short: &self.short, long: &self.long};
    }
}
impl HasPortal for Portal {
    fn get(&self) -> PortalInfo {
        return PortalInfo {to: &self.to[..]};
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Spawner { 
    pub name: Name,
    pub short: String,
    pub long: String,
}
impl HasRender for Spawner {
    fn get(&self) -> RenderInfo {
        return RenderInfo {short: &self.short, long: &self.long};
    }
}
impl HasSpawner for Spawner {
    fn get(&self) -> SpawnerInfo {
        return SpawnerInfo;
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub rooms: Vec<Room>,
    pub objects: Vec<Object>,
    pub containers: Vec<Container>,
    pub portals: Vec<Portal>,
    pub spawners: Vec<Spawner>,
}

pub fn example() -> Map {

    let mut exmap = Map {
        name: "Test Map".to_owned(),
        rooms: Vec::new(),
        objects: Vec::new(),
        containers: Vec::new(),
        portals: Vec::new(),
        spawners: Vec::new()
    };

    exmap.objects.push( Object {
        name: "object1".to_owned(),
        short: "small diamond".to_owned(),
        long: "The diamond emits an eerie glow.".to_owned(),
        movable: true
    });
    
    exmap.objects.push( Object {
        name: "object2".to_owned(),
        short: "pillar with a socket".to_owned(),
        long: "The pillar is made of stone, and has a socket at the top".to_owned(),
        movable: false
    });

    exmap.spawners.push( Spawner {
        name: "spawner1".to_owned(),
        short: "A crystal fountain".to_owned(),
        long: "The fountain seems connected to some other plane by dark magics".to_owned(),
    });

    exmap.portals.push( Portal {
        name: "door1".to_owned(),
        short: "A large wooden door".to_owned(),
        long: "The door looks old.".to_owned(),
        to: "room2".to_owned(),
    });

    exmap.rooms.push( Room {
        name: "room1".to_owned(),
        short: "The room is dark".to_owned(),
        long: "This is a crazy dark room".to_owned(),
        contains: vec!["object1".to_owned(), "spawner1".to_owned()],
        capacity: 100,
        walls: vec![("door1".to_owned(), Direction::North)]
    });
    
    exmap.rooms.push( Room {
        name: "room2".to_owned(),
        short: "The room is bright".to_owned(),
        long: "The room is lit by an strange glow.".to_owned(),
        contains: vec!["object2".to_owned()],
        capacity: 100,
        walls: vec![]
    });

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

