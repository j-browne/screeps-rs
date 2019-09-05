use crate::error::Res;
use screeps::{
    Position,
    HasPosition,
};
use std::{collections::VecDeque, fmt};
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
    Harvest { source_id: String },

}

js_serializable!(Job);
js_deserializable!(Job);

pub struct Creep {
    creep: screeps::Creep,
    role: Role,
    jobs: VecDeque<Job>,
}

impl Creep {
    pub fn new(creep: screeps::Creep) -> Res<Self> {
        let memory = creep.memory();
        let role = memory.get("role")?.unwrap_or_default();
        let jobs: Vec<Job> = memory.get("jobs")?.unwrap_or_default();
        let jobs = jobs.into();

        Ok( Self {
            creep,
            role,
            jobs,
        })
    }

    pub fn run(&mut self) {
        match self.jobs.front() {
            Some(Job::Move{pos}) => {
                self.creep.move_to(pos);
                if self.creep.pos().is_near_to(pos) {
                    self.jobs.pop_front();
                }
            },
            Some(Job::Harvest{..}) => {}
            _ => {}
        }
    }
}
