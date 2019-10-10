use self::actions::Action;
use crate::error::Res;
pub use roles::Role;
use screeps::Creep as ScreepsCreep;
use std::collections::VecDeque;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

mod actions;
mod roles;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreepMemory {
    pub home: String,
    #[serde(default)]
    pub role: Role,
    #[serde(default)]
    pub actions: VecDeque<Action>,
}

js_serializable!(CreepMemory);
js_deserializable!(CreepMemory);

pub struct Creep {
    obj: ScreepsCreep,
    memory: Option<CreepMemory>,
}

impl Drop for Creep {
    fn drop(&mut self) {
        screeps::memory::root()
            .path_set(&format!("creeps.{}", self.obj.name()), self.memory.take());
    }
}

impl Creep {
    pub fn new(obj: ScreepsCreep) -> Res<Self> {
        let memory = Some(
            screeps::memory::root()
                .get_path(&format!("creeps.{}", obj.name()))?
                .ok_or_else(|| format!("undefined or null creep memory for {}", obj.name()))?,
        );
        Ok(Self { obj, memory })
    }

    pub fn obj(&self) -> &ScreepsCreep {
        &self.obj
    }

    pub fn memory(&self) -> &CreepMemory {
        self.memory.as_ref().expect("creep.memory is not populated")
    }

    pub fn memory_mut(&mut self) -> &mut CreepMemory {
        self.memory.as_mut().expect("creep.memory is not populated")
    }

    pub fn run(&mut self) -> Res<()> {
        if let Some(action) = self.memory().actions.front().cloned() {
            action.run(self)?;
        }

        Ok(())
    }
}
