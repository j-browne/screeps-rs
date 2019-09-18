use crate::error::Res;
use screeps::{Position, ResourceType, Room as ScreepRoom};
use std::collections::HashMap;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoomMemory {
    #[serde(default)]
    pub mines: HashMap<ResourceType, Vec<Position>>,
}

js_serializable!(RoomMemory);
js_deserializable!(RoomMemory);

pub struct Room {
    pub obj: ScreepRoom,
    pub memory: RoomMemory,
}

impl Drop for Room {
    fn drop(&mut self) {
        screeps::memory::root()
            .path_set(&format!("rooms.{}", self.obj.name()), self.memory.clone());
    }
}

impl Room {
    pub fn new(obj: ScreepRoom) -> Res<Self> {
        let memory = screeps::memory::root()
            .get_path(&format!("rooms.{}", obj.name()))?
            .ok_or_else(|| format!("undefined or null room memory for {}", obj.name()))?;
        Ok(Self { obj, memory })
    }
}
