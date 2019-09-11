use self::actions::Action;
use crate::error::Res;
pub use roles::Role;
use screeps::Creep as ScreepCreep;
use std::collections::VecDeque;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

mod actions;
mod roles;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreepMemory {
    #[serde(default)]
    role: Role,
    #[serde(default)]
    actions: VecDeque<Action>,
}

js_serializable!(CreepMemory);
js_deserializable!(CreepMemory);

pub struct Creep {
    obj: ScreepCreep,
    memory: CreepMemory,
}

impl Drop for Creep {
    fn drop(&mut self) {
        screeps::memory::root()
            .path_set(&format!("creeps.{}", self.obj.name()), self.memory.clone());
    }
}

impl Creep {
    pub fn new(obj: ScreepCreep) -> Res<Self> {
        let memory = screeps::memory::root()
            .get_path(&format!("creeps.{}", obj.name()))?
            .ok_or_else(|| format!("undefined or null creep memory for {}", obj.name()))?;
        Ok(Self { obj, memory })
    }

    pub fn run(&mut self) -> Res<()> {
        if let Some(action) = self.memory.actions.front().cloned() {
            action.run(self)?;
        }

        Ok(())
    }
}
