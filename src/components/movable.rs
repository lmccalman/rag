// use super::super::command::UserCommand;
use anyhow::Result;
// use super::super::state::GameState;
use super::super::EntityID;
use std::collections::HashMap;


pub struct Movable {
    movable: HashMap<EntityID, bool>,
}

impl Movable {
    pub fn new() -> Movable {
        return Movable { movable: HashMap::new() };
    }

    pub fn add(&mut self, id: EntityID, m: bool) {
        self.movable.insert(id, m);
    }

}

