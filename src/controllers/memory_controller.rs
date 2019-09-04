use log::*;
use std::collections::HashSet;

pub struct MemoryController {}

impl MemoryController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.creeps()?;
        self.spawns()?;
        self.flags()?;

        Ok(())
    }

    fn creeps(&self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn spawns(&self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn flags(&self) -> Result<(), Box<dyn std::error::Error>> {
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
}
