use crate::{
    config::Config,
    error::Res,
};
use log::*;
use std::collections::HashSet;

pub struct MemoryController {}

impl MemoryController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cleanup(&self) -> Res<()> {
        self.cleanup_creeps()?;
        self.cleanup_spawns()?;
        self.cleanup_flags()?;

        Ok(())
    }

    fn cleanup_creeps(&self) -> Res<()> {
        let creeps_active: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();
        let creeps_memory = screeps::memory::root()
            .dict("creeps")?
            .ok_or("creeps does not exist in Memory")?;

        for mem_name in creeps_memory.keys() {
            if !creeps_active.contains(&mem_name) {
                info!("Clearing creep memory: {}", mem_name);
                creeps_memory.del(&mem_name);
            }
        }

        Ok(())
    }

    fn cleanup_spawns(&self) -> Res<()> {
        let spawns_active: HashSet<String> = screeps::game::spawns::keys().into_iter().collect();
        let spawns_memory = screeps::memory::root()
            .dict("spawns")?
            .ok_or("spawns does not exist in Memory")?;

        for mem_name in spawns_memory.keys() {
            if !spawns_active.contains(&mem_name) {
                info!("Clearing spawn memory: {}", mem_name);
                spawns_memory.del(&mem_name);
            }
        }

        Ok(())
    }

    fn cleanup_flags(&self) -> Res<()> {
        let flags_active: HashSet<String> = screeps::game::flags::keys().into_iter().collect();
        let flags_memory = screeps::memory::root()
            .dict("flags")?
            .ok_or("flags does not exist in Memory")?;

        for mem_name in flags_memory.keys() {
            if !flags_active.contains(&mem_name) {
                info!("Clearing flag memory: {}", mem_name);
                flags_memory.del(&mem_name);
            }
        }

        Ok(())
    }


    pub fn config(&self) -> Res<Config> {
        Ok(screeps::memory::root()
            .get("config")?
            .ok_or("undefined or null config")?)
    }

    pub fn update(&self) -> Res<()> {
        unimplemented!()
    }
}
