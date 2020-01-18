use std::collections::HashMap;
use super::super::EntityID;

pub struct Portal {
    // where is the orientation of an object in a room
    pub portals: HashMap<EntityID, EntityID>,
}

impl Portal {

    pub fn new() -> Portal {
        return Portal {portals: HashMap::new()};
    }
    pub fn add(&mut self, id: EntityID, to: EntityID) {
        self.portals.insert(id, to);
    }
}
