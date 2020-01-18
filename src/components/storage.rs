use super::super::{EntityID, Direction};
use std::collections::HashMap;
use super::super::util::{get_result, get_result_mut};
use anyhow::{Result, anyhow};

type Chest = HashMap<EntityID, ()>;
type FacetStore = HashMap<EntityID, Direction>;
type CharStore = HashMap<EntityID, ()>;

pub enum Location {
    Chest(EntityID),
    Facet(EntityID, Direction),
    Character(EntityID)
}

pub struct Storage {
    chest: HashMap<EntityID, Chest>,
    facet: HashMap<EntityID, FacetStore>,
    chars: HashMap<EntityID, CharStore>,
    capacity: HashMap<EntityID, usize>,
    lookup: HashMap<EntityID, Location>
}

impl Storage {
    pub fn new() -> Storage { 
        return Storage {
            chest: HashMap::new(), 
            facet: HashMap::new(),
            chars: HashMap::new(),
            capacity: HashMap::new(),
            lookup: HashMap::new(),
        }; 
    }

    pub fn add_chest(&mut self, id: EntityID, capacity: &usize) {
        self.chest.insert(id, HashMap::new());
        self.capacity.insert(id, capacity.clone());
    }
    pub fn try_into_chest(&mut self, chest: &EntityID, obj: &EntityID) -> Result<()> {
        let c = get_result_mut(chest, &mut self.chest)?;
        let n = c.len(); 
        if n < self.capacity[chest] {
            c.insert(obj.clone(), () );
            self.lookup.insert(obj.clone(), Location::Chest(chest.clone()) );
        }
        else {
            return Err(anyhow!("No space in chest"));
        }
        return Ok(());
    }

    pub fn add_facet(&mut self, id: &EntityID) {
        self.facet.insert(id.clone(), HashMap::new());
    }

    pub fn into_facet(&mut self, facetstore: &EntityID, dir: &Direction, obj: &EntityID) -> Result<()> {
        let f = get_result_mut(facetstore, &mut self.facet)?;
        f.insert(obj.clone(), dir.clone());
        self.lookup.insert(obj.clone(), Location::Facet(facetstore.clone(), dir.clone()));
        return Ok(());
    }

    pub fn add_character(&mut self, id: EntityID) {
        self.chars.insert(id, HashMap::new());
    }

    pub fn into_character(&mut self, charstore: &EntityID, obj: &EntityID) -> Result<()> {
        let c = get_result_mut(charstore, &mut self.chars)?;
        c.insert(obj.clone(), ());
        self.lookup.insert(obj.clone(), Location::Character(charstore.clone()));
        return Ok(());
    }

}
