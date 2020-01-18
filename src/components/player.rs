use std::collections::HashMap;
use super::super::EntityID;

pub struct Player {
    pub players: HashMap<EntityID, Player>,
}

impl Player {

    pub fn new() -> Player {
        return Player {players: HashMap::new()};
    }


}

