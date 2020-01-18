use std::collections::{HashSet, HashMap};
use super::EntityID;
use super::util;
use anyhow::{Result, anyhow};


// pub struct EntityInfo<'a> {
//     pub name: &'a str
// }
// pub trait HasEntity {
//     fn get(&self) -> EntityInfo;
// }

pub struct EntityManager {
    idgen: u64,
    current: HashMap<EntityID, String>,
    lookup: HashMap<String, EntityID>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        return EntityManager { idgen: 0, current: HashMap::new(), lookup: HashMap::new() };
    }

    pub fn add(&mut self, name: &str) -> EntityID {
         
        let mut found = false;
        while !found {
            self.idgen += 1;
            if !self.current.contains_key(&self.idgen) {
                found = true;
                self.current.insert(self.idgen, name.to_string());
                self.lookup.insert(name.to_string(), self.idgen);
            }
        }
        return self.idgen;
    }
    pub fn exists(&mut self, id: &EntityID) -> bool {
        return self.current.contains_key(id);
    }
    pub fn name<'a>(&'a mut self, id: &'a EntityID) -> Result<&'a str> {
        let res = util::get_result(id, &self.current).map(|x| &x[..]); 
        return res;
    }
    pub fn id(&mut self, name: &str) -> Result<EntityID> {
        let res = self.lookup.get(name).ok_or(anyhow!("Key not found in hashmap"));
        return res.map(|x| x.clone());
    }
}

