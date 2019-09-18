use crate::{creeps::Role, error::Res};
use screeps::constants::Part;
use std::collections::HashMap;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub roles_to_spawn: HashMap<String, Vec<(Role, String)>>,
    pub equip: HashMap<String, Vec<Part>>,
}

impl Config {
    pub fn new() -> Res<Self> {
        Ok(screeps::memory::root()
            .get("config")?
            .ok_or("undefined or null config memory")?)
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        screeps::memory::root().set("config", self.clone());
    }
}

js_serializable!(Config);
js_deserializable!(Config);
