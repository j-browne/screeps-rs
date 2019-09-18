use crate::{config::Config, creeps::Role, error::Res, names::get_random_name, rooms::Room};
use log::*;
use screeps::{find, memory::MemoryReference, ReturnCode, SpawnOptions, StructureSpawn};
use std::collections::HashMap;

pub struct Mayor<'a> {
    pub room: Room,
    pub config: &'a Config,
}

impl<'a> Mayor<'a> {
    pub fn new(room: Room, config: &'a Config) -> Self {
        Self { room, config }
    }

    pub fn run(self) -> Res<()> {
        self.determine_spawns()?;

        Ok(())
    }

    pub fn determine_spawns(&self) -> Res<()> {
        // If there's nothing to spawn, just return
        let room_name = self.room.obj.name().to_array_string();
        if !self.config.roles_to_spawn.contains_key(room_name.as_str()) {
            return Ok(());
        }

        // If there are no spawns, just return
        let mut spawns = self.room.obj.find(find::MY_SPAWNS);
        if spawns.len() == 0 {
            return Ok(());
        }

        // If the spawn is currently spawning, just return
        if spawns[0].is_spawning() {
            return Ok(());
        }

        // Get the number of creeps in each role by iterating
        // through the creeps in the room and incrementing the
        // counter for that role
        let mut current_roles = HashMap::<Role, u8>::new();
        for creep in self.room.obj.find(find::MY_CREEPS) {
            if let Ok(Some(role)) = creep.memory().get("role") {
                let counter = current_roles.entry(role).or_insert(0);
                *counter += 1;
            }
        }

        let roles_to_spawn = &self.config.roles_to_spawn[room_name.as_str()];
        for (role, equip_name) in roles_to_spawn {
            let counter = current_roles.entry(*role).or_insert(0);

            if *counter == 0 {
                match self.spawn(spawns.remove(0), *role, equip_name) {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        warn!("{}", e);
                        continue;
                    }
                }
            }

            *counter -= 1;
        }

        Ok(())
    }

    pub fn spawn(&self, spawn: StructureSpawn, role: Role, equip_name: &str) -> Res<ReturnCode> {
        let body = self
            .config
            .equip
            .get(equip_name)
            .ok_or_else(|| format!("{} not found in equip", role))?;
        let name = get_random_name();
        // FIXME: create a CreepMemory
        let memory = MemoryReference::new();
        memory.set("role", format!("{}", role));
        let options = SpawnOptions::new().memory(memory);

        let ret = spawn.spawn_creep_with_options(body, &name, &options);
        if ret == ReturnCode::Ok {
            info!("Spawning {} ({})", name, role);
        }

        Ok(ret)
    }
}
