#[macro_use]
extern crate serde_derive;

use crate::{
    controllers::{MemoryController, SpawnController},
    error::Res,
    creeps::run_creep,
};
use log::*;
use stdweb::js;

mod config;
mod controllers;
mod creeps;
mod error;
mod logging;

fn main() {
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop_catch};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop_catch() {
    match game_loop() {
        Ok(()) => {}
        Err(e) => {
            warn!("{}", e.description());
            if let Some(s) = e.source() {
                warn!("cause: {}", s);
            }
        }
    }
}

fn game_loop() -> Res<()> {
    let mut memory_controller = MemoryController::new(screeps::memory::root())?;
    memory_controller.cleanup()?;

    let config = memory_controller.take_config()?;
    let mut creeps_memory = memory_controller.take_creeps()?;

    for room in screeps::game::rooms::values() {
        SpawnController::new(room, &config).run();
    }

    for creep in screeps::game::creeps::values() {
        run_creep(creep, &mut creeps_memory)?;
    }

    memory_controller.set_config(config);
    memory_controller.set_creeps(creeps_memory);
    memory_controller.update();
    Ok(())
}
