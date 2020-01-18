use std::collections::HashMap;
use super::super::EntityID;

pub struct Location {
    // where is the location of an object
    pub locations: HashMap<EntityID, EntityID>,
}

impl Location {
    
    pub fn new() -> Location {
        return Location {locations: HashMap::new()};
    }

}
