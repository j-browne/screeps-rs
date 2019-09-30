use crate::{creeps::Role, error::Res};
use screeps::constants::Part;
use std::collections::HashMap;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfigMemory {
    #[serde(default)]
    pub roles_to_spawn: HashMap<String, Vec<(Role, String)>>,
    #[serde(default)]
    pub equip: HashMap<String, Vec<Part>>,
}

js_serializable!(ConfigMemory);
js_deserializable!(ConfigMemory);

#[derive(Clone, Debug)]
pub struct Config {
    memory: Option<ConfigMemory>,
}

impl Config {
    pub fn new() -> Res<Self> {
        let memory = Some(
            screeps::memory::root()
                .get("config")?
                .ok_or("undefined or null config memory")?,
        );
        Ok(Self { memory })
    }

    pub fn memory(&self) -> &ConfigMemory {
        self.memory
            .as_ref()
            .expect("config.memory is not populated")
    }

    pub fn memory_mut(&mut self) -> &mut ConfigMemory {
        self.memory
            .as_mut()
            .expect("config.memory is not populated")
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        let memory = self.memory.take();
        screeps::memory::root().set("config", memory);
    }
}
