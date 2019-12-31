use console::Term;   
use super::state::GameState;
use super::render::render;


pub enum Movement {
    North,
    South,
    East,
    West
}

pub enum System {
    Quit,
    Initialise
}

pub enum Command {
    System(System),
    Movement(Movement),
    Unknown(String)
}

fn movement(c: &Command, s: &mut GameState){


}

pub fn process(c: &Command, s: &mut GameState, term: &Term) {

    movement(c, s);
    // this haapens LAST
    render(c, s, term);

}
