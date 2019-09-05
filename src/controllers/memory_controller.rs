use crate::{
    config::Config,
    error::Res,
    creeps::CreepMemory,
};
use log::*;
use std::{
    collections::{HashMap, HashSet},
    ops::Drop,
};
use screeps::memory::MemoryReference;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};


pub struct MemoryController {
    mem: MemoryReference,
    config: Config,
    creeps: HashMap<String, CreepMemory>,
}

impl MemoryController {
    pub fn new(mem: MemoryReference) -> Res<Self> {
        let config = mem
            .get("config")?
            .ok_or("undefined or null config")?;
        let creeps = mem
            .get("creeps")?
            .ok_or("undefined or null creep memory")?;

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

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn creeps(&self) -> &HashMap<String, CreepMemory> {
        &self.creeps
    }

    pub fn creeps_mut(&mut self) -> &mut HashMap<String, CreepMemory> {
        &mut self.creeps
    }

    pub fn update(&self) {
        self.mem.set("config", self.config);
        self.mem.set("creeps", self.creeps);
    }
}

impl Drop for MemoryController {
    fn drop(&mut self) {
        self.update()
    }
}