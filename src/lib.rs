use std::collections::HashMap;

use screeps_arena::{constants, game, prototypes, BodyPart, HasHits};
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
    let my_damaged_creeps = my_creeps
        .clone()
        .into_iter()
        .filter(|my_creep| my_creep.hits() < my_creep.hits_max())
        .collect::<Vec<_>>();
    let enemy_creeps = creeps
        .iter()
        .filter(|creep| !creep.my())
        .collect::<Vec<_>>();

    for creep in my_creeps {
        if creep
            .body()
            .iter()
            .any(|body_part| body_part.part() == constants::Part::Attack)
            && creep.attack(enemy_creeps[0]) == constants::ReturnCode::NotInRange
        {
            creep.move_to(enemy_creeps[0], None);
        }
        if creep
            .body()
            .iter()
            .any(|body_part| body_part.part() == constants::Part::RangedAttack)
            && creep.ranged_attack(enemy_creeps[0]) == constants::ReturnCode::NotInRange
        {
            creep.move_to(enemy_creeps[0], None);
        }
        if creep
            .body()
            .iter()
            .any(|body_part| body_part.part() == constants::Part::Heal)
            && !my_damaged_creeps.is_empty()
            && creep.heal(my_damaged_creeps[0]) == constants::ReturnCode::NotInRange
        {
            creep.move_to(my_damaged_creeps[0], None);
        }
    }
}
