use crate::{
    config::Config,
    error::Res,
    creeps::CreepMemory,
};
use log::*;
use std::{
    collections::{HashMap, HashSet},
};
use screeps::memory::MemoryReference;


pub struct MemoryController {
    mem: MemoryReference,
    config: Option<Config>,
    creeps: Option<HashMap<String, CreepMemory>>,
}

impl MemoryController {
    pub fn new(mem: MemoryReference) -> Res<Self> {
        let config = Some(mem
            .get("config")?
            .ok_or("undefined or null config")?
        );
        let creeps = Some(mem
            .get("creeps")?
            .ok_or("undefined or null creep memory")?
        );

        Ok(Self{
            mem,
            config,
            creeps,
        })
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

    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn take_config(&mut self) -> Res<Config> {
        Ok(self.config.take().ok_or("config is not populated")?)
    }

    pub fn set_creeps(&mut self, creeps: HashMap<String, CreepMemory>) {
        self.creeps = Some(creeps);
    }

    pub fn take_creeps(&mut self) -> Res<HashMap<String, CreepMemory>> {
        Ok(self.creeps.take().ok_or("creeps is not populated")?)
    }

    pub fn update(self) {
        if let Some(config) = self.config {
            self.mem.set("config", config);
        }
        if let Some(creeps) = self.creeps {
            self.mem.set("creeps", creeps);
        }
    }
}