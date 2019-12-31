use super::map;
use std::collections::{HashMap, HashSet};

type EntityID = i64;

type NameLookup = HashMap<map::EntityName, EntityID>;
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct EntityID(pub i64);
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct Location(pub i64);

type Name = String;
type ShortDescription = String;
type LongDescription = String;
type Container = HashSet<EntityID>;

#[derive(Debug)]
pub struct Portal { pub from: EntityID, pub to: EntityID }
#[derive(Debug)]
pub struct Player { pub location: EntityID }


#[derive(Debug)]
pub struct GameState {
    pub game_name: String,
    pub names: HashMap<EntityID, Name>,
    pub shorts: HashMap<EntityID, ShortDescription>,
    pub longs: HashMap<EntityID, LongDescription>,
    // where is the location of an object
    pub locations: HashMap<EntityID, EntityID>,
    pub portals: HashMap<EntityID, Portal>,
    // what objects are in the container
    pub containers: HashMap<EntityID, Container>,
    // can have players inserted
    pub travelables: HashMap<EntityID, ()>,
    // can be moved around
    pub movables: HashMap<EntityID, ()>,
    pub capacities: HashMap<EntityID, i64>,
    pub player: Player,
}

pub struct IDGen(std::ops::Range<i64>);

impl IDGen {
    pub fn new() -> IDGen {
        return IDGen(std::ops::Range {start: 0, end: 2^32});
    }
    
    pub fn next_entity(&mut self) -> EntityID {
        return self.0.next().unwrap();
    }
    pub fn next_location(&mut self) -> EntityID {
        return self.0.next().unwrap();
    }


}

impl GameState {

    fn init_room(&mut self, r: &map::Room, id: EntityID, lookup: &mut NameLookup) {
        self.names.insert(id, r.name.0.clone());
        lookup.insert(r.name.clone(), id);
        self.containers.insert(id, HashSet::new());
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

    fn add_room(&mut self, r: &map::Room, lookup: &NameLookup) {
        let id = lookup[&r.name];
        self.shorts.insert(id, r.short.clone());
        if let Some(l) = &r.long { 
            self.longs.insert(id, l.clone()); 
        };
        self.travelables.insert(id, ());
    }

    fn add_container(&mut self, c: &map::Container, lookup: &NameLookup) {
        let id = lookup[&c.name];
        self.shorts.insert(id, c.short.clone());
        if let Some(l) = &c.long {
            self.longs.insert(id, l.clone());
        };
        let loc = lookup[&c.location];
        self.locations.insert(id, loc);
        if c.movable {
            self.movables.insert(id, ());
        }
        self.capacities.insert(id, c.capacity);
        if let Some(s) = self.containers.get_mut(&loc) {
            s.insert(id);
        };
    }
    
    fn add_object(&mut self, o: &map::Object, lookup: &NameLookup) {
        let id = lookup[&o.name];
        self.shorts.insert(id, o.short.clone());
        if let Some(l) = &o.long {
            self.longs.insert(id, l.clone());
        };
        if o.movable {
            self.movables.insert(id, ());
        }
        let loc = lookup[&o.location];
        self.locations.insert(id, loc);
        if let Some(s) = self.containers.get_mut(&loc) {
            s.insert(id);
        };
    }

    fn add_portal(&mut self, p: &map::Portal, lookup: &NameLookup) {
        let id = lookup[&p.name];
        self.shorts.insert(id, p.short.clone());
        if let Some(l) = &p.long {
            self.longs.insert(id, l.clone());
        };
        let loc = lookup[&p.location];
        self.locations.insert(id, loc);
        if let Some(s) = self.containers.get_mut(&loc) {
            s.insert(id);
        };
        let from = lookup[&p.from];
        let to = lookup[&p.to];
        self.portals.insert(id, Portal { from, to });

        // portals must be in rooms


    }
    
    pub fn load(m: &map::Map) -> GameState {
        
        let mut gen = IDGen::new();
        let mut name_lookup: HashMap<map::EntityName, EntityID > = HashMap::new();
        let player_loc = gen.next_entity();

        let mut s = GameState {
            game_name: m.name.clone(),
            names: HashMap::new(),
            shorts: HashMap::new(),
            longs: HashMap::new(),
            locations: HashMap::new(),
            portals: HashMap::new(),
            containers: HashMap::new(),
            movables: HashMap::new(),
            capacities: HashMap::new(),
            travelables: HashMap::new(),
            player: Player {location: player_loc }
        };

        // step 1: give everything ids
        for r in &m.rooms {
            let id = if r.name == m.player.location {
                player_loc
            } else {
                gen.next_entity()
            };
            s.init_room(r, id, &mut name_lookup);
        }
        for c in &m.containers {
            let id = gen.next_entity();
            s.init_container(c, id, &mut name_lookup);
        }
        for o in &m.objects {
            let id = gen.next_entity();
            s.init_object(o, id, &mut name_lookup);
        }
        for p in &m.portals {
            let id = gen.next_entity();
            s.init_portal(p, id, &mut name_lookup);
        }
        
        // step 2: do all the references and other stuff
        for r in &m.rooms { s.add_room(r, &name_lookup); }
        for c in &m.containers { s.add_container(c, &name_lookup); }
        for o in &m.objects { s.add_object(o, &name_lookup); }
        for p in &m.portals { s.add_portal(p, &name_lookup); }

        return s;    
    }
}
