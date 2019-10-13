use crate::{error::Res, Id};
use screeps::{Position, ResourceType, Room as ScreepsRoom};
use std::collections::HashMap;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoomMemory {
    #[serde(default)]
    pub mines: HashMap<ResourceType, Vec<Position>>,
    #[serde(default)]
    pub extension_containers: Vec<Id>,
    #[serde(default)]
    pub extension_spots: Vec<Position>,
    #[serde(default)]
    pub lab_spots: HashMap<ResourceType, Id>,
    #[serde(default)]
    pub forts: Vec<Position>,
    #[serde(default)]
    pub repair_blacklist: Vec<Id>,
}

js_serializable!(RoomMemory);
js_deserializable!(RoomMemory);

pub struct Room {
    obj: ScreepsRoom,
    memory: Option<RoomMemory>,
}

impl Drop for Room {
    fn drop(&mut self) {
        screeps::memory::root().path_set(&format!("rooms.{}", self.obj.name()), self.memory.take());
    }
}

impl Room {
    pub fn new(obj: ScreepsRoom) -> Res<Self> {
        let memory = Some(
            screeps::memory::root()
                .dict_or_create("rooms")
                .map_err(|_| "UnexpectedTypeError")?
                .get(&obj.name().to_array_string())?
                .ok_or_else(|| format!("undefined or null room memory for {}", obj.name()))?,
        );
        Ok(Self { obj, memory })
    }

    pub fn obj(&self) -> &ScreepsRoom {
        &self.obj
    }

    pub fn memory(&self) -> &RoomMemory {
        self.memory.as_ref().expect("room.memory is not populated")
    }

    pub fn memory_mut(&mut self) -> &mut RoomMemory {
        self.memory.as_mut().expect("room.memory is not populated")
    }
}
