use crate::error::Res;
use screeps::{
    Creep,
    Position,
    HasPosition,
    objects::Source,
    local::ObjectId,
};
use std::{collections::{HashMap, VecDeque}, fmt};
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "")]
    Generic,
    #[serde(rename = "H")]
    Harvester,
    #[serde(rename = "T")]
    Transporter,
    #[serde(rename = "B")]
    Builder,
    #[serde(rename = "U")]
    Upgrader,
    #[serde(rename = "A")]
    Attacker,
}

js_serializable!(Role);
js_deserializable!(Role);

impl Default for Role {
    fn default() -> Self {
        Role::Generic
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Generic => write!(f, ""),
            Role::Harvester => write!(f, "H"),
            Role::Transporter => write!(f, "T"),
            Role::Builder => write!(f, "B"),
            Role::Upgrader => write!(f, "U"),
            Role::Attacker => write!(f, "A"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Job {
    Move { pos: Position },
    Harvest { source_id: ObjectId<Source> },
}

js_serializable!(Job);
js_deserializable!(Job);

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreepMemory {
    #[serde(default)]
    role: Role,
    #[serde(default)]
    jobs: VecDeque<Job>,
}

js_serializable!(CreepMemory);
js_deserializable!(CreepMemory);

pub fn run_creep(creep: Creep, creeps_memory: &mut HashMap<String, CreepMemory>) -> Res<()> {
    let memory = creeps_memory.entry(creep.name()).or_default();
    let jobs = &mut memory.jobs;
    match jobs.front().clone() {
        Some(Job::Move {pos}) => {
            if *pos == creep.pos() {
                jobs.pop_front();
            } else {
                creep.move_to(pos);
            }
        }
        Some(Job::Harvest {source_id}) => {
            let source: Source = screeps::game::get_object_typed(*source_id)?.ok_or("undefined or null source")?;
            if creep.pos().is_near_to(&source) {
            }
        }
        None => {}
    }

    Ok(())
}