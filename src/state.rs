use std::fmt;
use std::hash::Hash;
use super::map;
use std::collections::{HashMap, HashSet};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use anyhow::{Result, anyhow};

type EntityID = u64;
type NameLookup = HashMap<map::EntityName, EntityID>;
type Name = String;
type ShortDescription = String;
type LongDescription = String;
type Container = HashSet<EntityID>;
type Travelable = HashSet<EntityID>;

#[derive(Debug)]
pub struct Portal { pub from: EntityID, pub to: EntityID}

#[derive(Debug)]
pub struct Player { pub location: EntityID }

pub struct IDGen(u64);

impl IDGen {
    pub fn new() -> IDGen {
        return IDGen(0);
    }
    pub fn next(&mut self, entities: &HashSet<EntityID>) -> EntityID {
         
        let mut found = false;
        while !found {
            self.0 += 1;
            if !entities.contains(&self.0) {
                found = true;
            }
        }
        return self.0;
    }

}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum Direction {
    North = 0,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "north"),
            Direction::Northeast => write!(f, "northeast"),
            Direction::East => write!(f, "east"),
            Direction::Southeast => write!(f, "southeast"),
            Direction::South => write!(f, "south"),
            Direction::Southwest => write!(f, "southwest"),
            Direction::West => write!(f, "west"),
            Direction::Northwest => write!(f, "northwest"),
        }
    }
}

impl Direction {
    fn from_map(d: map::RoomSurface) -> Direction {
        let direction_primitive : u8 = d.into();
        return Direction::try_from(direction_primitive).unwrap();
    }


}


type DirectionTable = HashMap<Direction, EntityID>;


#[derive(Debug)]
pub struct GameState {
    pub game_name: String,
    pub entities: HashSet<EntityID>,
    pub names: HashMap<EntityID, Name>,
    pub shorts: HashMap<EntityID, ShortDescription>,
    pub longs: HashMap<EntityID, LongDescription>,
    // where is the location of an object
    pub locations: HashMap<EntityID, EntityID>,
    // where is the orientation of an object in a room
    pub orientations: HashMap<EntityID, Direction>,
    pub portals: HashMap<EntityID, Portal>,
    // what objects are in the container
    pub containers: HashMap<EntityID, Container>,
    // what exists in faceted storage
    pub faceted: HashMap<EntityID, DirectionTable>,
    // can have players inserted
    pub travelables: HashMap<EntityID, ()>,
    // can be moved around
    pub movables: HashMap<EntityID, ()>,
    pub capacities: HashMap<EntityID, i64>,
    pub players: HashMap<EntityID, Player>,
}

pub fn get_result<'a, K: Eq + Hash, V>(id: &'a K, 
                                       map: &'a HashMap<K, V>, 
                                       desc: &str) -> Result<&'a V> {
    let obj = (*map).get(id).ok_or(anyhow!("Can't find {}", desc))?;
    return Ok(obj);
}

pub fn get_mut_result<'a, K: Eq + Hash, V>(id: &'a K, 
                                           map: &'a mut HashMap<K, V>, 
                                           desc: &str) -> Result<&'a mut V> {
    let obj = map.get_mut(id).ok_or(anyhow!("Can't find {}", desc))?;
    return Ok(obj);
}


impl GameState {

    fn init_room(&mut self, r: &map::Room, id: EntityID, lookup: &mut NameLookup) {
        self.names.insert(id, r.name.0.clone());
        lookup.insert(r.name.clone(), id);
        self.containers.insert(id, HashSet::new());
        self.faceted.insert(id, HashMap::new());
    }
    
    fn init_container(&mut self, c: &map::Container, id: EntityID, lookup: &mut NameLookup) {
        self.names.insert(id, c.name.0.clone());
        lookup.insert(c.name.clone(), id);
        self.containers.insert(id, HashSet::new());
    }

    fn init_object(&mut self, o: &map::Object, id: EntityID, lookup: &mut NameLookup) {
        self.names.insert(id, o.name.0.clone());
        lookup.insert(o.name.clone(), id);
    }
    
    fn init_portal(&mut self, p: &map::Portal, id: EntityID, lookup: &mut NameLookup) {
        self.names.insert(id, p.name.0.clone());
        lookup.insert(p.name.clone(), id);
    }

    fn add_room(&mut self, r: &map::Room, lookup: &NameLookup) -> Result<()> {
        let id = get_result(&r.name, lookup, &"room")?.clone();
        self.shorts.insert(id, r.short.clone());
        if let Some(l) = &r.long { 
            self.longs.insert(id, l.clone()); 
        };
        self.travelables.insert(id, ());
        return Ok(());
    }

    fn add_container(&mut self, c: &map::Container, lookup: &NameLookup) -> Result<()> {
        let id = get_result(&c.name, lookup, &"container")?.clone();
        self.shorts.insert(id, c.short.clone());
        if let Some(l) = &c.long {
            self.longs.insert(id, l.clone());
        };
        let loc = get_result(&c.location, lookup, "container location")?.clone();
        self.locations.insert(id, loc);
        if c.movable {
            self.movables.insert(id, ());
        }
        self.capacities.insert(id, c.capacity);
        let cont = get_mut_result(&loc, &mut self.containers, "container table")?;
        cont.insert(id);
        return Ok(());
    }
    
    fn add_object(&mut self, o: &map::Object, lookup: &NameLookup) -> Result<()> {
        let id = get_result(&o.name, lookup, &"object")?.clone();
        self.shorts.insert(id, o.short.clone());
        if let Some(l) = &o.long {
            self.longs.insert(id, l.clone());
        };
        if o.movable {
            self.movables.insert(id, ());
        }
        let loc = get_result(&o.location, lookup, "object location")?.clone();
        self.locations.insert(id, loc);
        let cont = get_mut_result(&loc, &mut self.containers, "container table")?;
        cont.insert(id);
        return Ok(());
    }

    fn add_portal(&mut self, p: &map::Portal, lookup: &NameLookup) -> Result<()> {
        let id = get_result(&p.name, lookup, &"portal")?.clone();
        self.shorts.insert(id, p.short.clone());
        if let Some(l) = &p.long {
            self.longs.insert(id, l.clone());
        };
        let loc = get_result(&p.location, lookup, "portal location")?.clone();
        self.locations.insert(id, loc);
        let orientation = Direction::from_map(p.surface);
        self.orientations.insert(id, orientation.clone());
        let facets = get_mut_result(&loc, &mut self.faceted, "portal facet table")?;
        facets.insert(orientation.clone(), id);
        let from = get_result(&p.from, lookup, "portal from")?.clone();
        let to = get_result(&p.to, lookup, "portal to")?.clone();
        self.portals.insert(id, Portal { from, to });
        return Ok(());
    }
    
    pub fn load(m: &map::Map) -> Result<GameState> {
        
        let mut gen = IDGen::new();
        let mut name_lookup: HashMap<map::EntityName, EntityID > = HashMap::new();

        let mut s = GameState {
            game_name: m.name.clone(),
            entities: HashSet::new(),
            names: HashMap::new(),
            shorts: HashMap::new(),
            longs: HashMap::new(),
            locations: HashMap::new(),
            orientations: HashMap::new(),
            portals: HashMap::new(),
            faceted: HashMap::new(),
            containers: HashMap::new(),
            movables: HashMap::new(),
            capacities: HashMap::new(),
            travelables: HashMap::new(),
            players: HashMap::new()
        };

        // step 1: give everything ids
        for r in &m.rooms {
            let id = if r.name == m.player.location {
                player_loc
            } else {
                gen.next()?
            };
            s.init_room(r, id, &mut name_lookup);
        }
        for c in &m.containers {
            let id = gen.next()?;
            s.init_container(c, id, &mut name_lookup);
        }
        for o in &m.objects {
            let id = gen.next()?;
            s.init_object(o, id, &mut name_lookup);
        }
        for p in &m.portals {
            let id = gen.next()?;
            s.init_portal(p, id, &mut name_lookup);
        }
        
        // step 2: do all the references and other stuff
        for r in &m.rooms { s.add_room(r, &name_lookup)?; }
        for c in &m.containers { s.add_container(c, &name_lookup)?; }
        for o in &m.objects { s.add_object(o, &name_lookup)?; }
        for p in &m.portals { s.add_portal(p, &name_lookup)?; }

        return Ok(s);    
    }
}
