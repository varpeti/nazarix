use screeps_arena::{game, prototypes};
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
    let flags = game::utils::get_objects_by_prototype(prototypes::FLAG);
    creeps[0].move_to(&flags[0], None);
}
