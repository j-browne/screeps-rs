use crate::{creeps::Role, error::Res};
use screeps::constants::Part;
use std::collections::HashMap;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Inner {
    roles_to_spawn: HashMap<String, Vec<(Role, String)>>,
    equip: HashMap<String, Vec<Part>>,
}

js_serializable!(Inner);
js_deserializable!(Inner);

#[derive(Clone, Debug)]
pub struct Config {
    inner: Option<Inner>,
}

impl Config {
    pub fn new() -> Res<Self> {
        Ok(Self {
            inner: Some(
                screeps::memory::root()
                    .get("config")?
                    .ok_or("undefined or null config memory")?,
            ),
        })
    }

    pub fn roles_to_spawn(&self) -> &HashMap<String, Vec<(Role, String)>> {
        &self
            .inner
            .as_ref()
            .expect("config.inner is not populated")
            .roles_to_spawn
    }

    pub fn roles_to_spawn_mut(&mut self) -> &mut HashMap<String, Vec<(Role, String)>> {
        &mut self
            .inner
            .as_mut()
            .expect("config.inner is not populated")
            .roles_to_spawn
    }

    pub fn equip(&self) -> &HashMap<String, Vec<Part>> {
        &self
            .inner
            .as_ref()
            .expect("config.inner is not populated")
            .equip
    }

    pub fn equip_mut(&mut self) -> &mut HashMap<String, Vec<Part>> {
        &mut self
            .inner
            .as_mut()
            .expect("config.inner is not populated")
            .equip
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        let inner = self.inner.take();
        screeps::memory::root().set("config", inner);
    }
}
