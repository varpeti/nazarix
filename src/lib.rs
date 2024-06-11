use screeps_arena::{constants, game, prototypes};
use wasm_bindgen::prelude::*;

mod logging;

fn setup() {
    logging::setup_logging(logging::Info);
}

#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();
    if tick == 1 {
        setup();
    }
    let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
    let my_creeps = creeps.iter().filter(|creep| creep.my()).collect::<Vec<_>>();
    let enemy_creeps = creeps
        .iter()
        .filter(|creep| !creep.my())
        .collect::<Vec<_>>();
    if my_creeps[0].attack(enemy_creeps[0]) == constants::ReturnCode::NotInRange {
        my_creeps[0].move_to(enemy_creeps[0], None);
    }
}
