use crate::{config::Config, creeps::Role};
use log::*;
use rand::{rngs::SmallRng, seq::IteratorRandom};
use rand_core::SeedableRng;
use screeps::{find, memory::MemoryReference, ReturnCode, Room, SpawnOptions, StructureSpawn};
use std::collections::HashMap;

mod names;

pub struct SpawnController<'c> {
    room: Room,
    config: &'c Config,
}

impl<'c> SpawnController<'c> {
    pub fn new(room: Room, config: &'c Config) -> SpawnController {
        SpawnController { room, config }
    }

    pub fn run(&self) {
        // If there's nothing to spawn, just return
        let room_name = self.room.name().to_array_string();
        if !self.config.roles_to_spawn.contains_key(room_name.as_str()) {
            return;
        }

        // If there are no spawns, just return
        let mut spawns = self.room.find(find::MY_SPAWNS);
        if spawns.len() == 0 {
            return;
        }

        // If the spawn is currently spawning, just return
        if spawns[0].is_spawning() {
            return;
        }

        // Get the number of creeps in each role by iterating
        // through the creeps in the room and incrementing the
        // counter for that role
        let mut current_roles = HashMap::<Role, u8>::new();
        for creep in self.room.find(find::MY_CREEPS) {
            if let Ok(Some(role)) = creep.memory().get("role") {
                let counter = current_roles.entry(role).or_insert(0);
                *counter += 1;
            }
        }

        let roles_to_spawn = &self.config.roles_to_spawn[room_name.as_str()];
        for role in roles_to_spawn {
            let counter = current_roles.entry(*role).or_insert(0);

            if *counter == 0 {
                match self.spawn(spawns.remove(0), *role) {
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
    }

    pub fn spawn(&self, spawn: StructureSpawn, role: Role) -> Result<ReturnCode, String> {
        let body = self
            .config
            .equip
            .get(&role)
            .ok_or_else(|| format!("{} not found in equip", role))?;
        let name = get_name();
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

pub fn get_name() -> String {
    use self::names::NAMES;
    let mut rng = SmallRng::seed_from_u64(screeps::game::time().into());
    NAMES
        .iter()
        .filter(|n| !screeps::game::creeps::hashmap().contains_key(**n))
        .choose(&mut rng)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("Creep_{}", screeps::game::time()))
}
