use std::collections::HashMap;
use super::super::EntityID;

pub struct Spawner {
    pub spawners: HashMap<EntityID, ()>,
}
