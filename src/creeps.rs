use screeps::Position;
use std::fmt;
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
    role: Role,
    job: Option<Job>,
}

impl Creep {
    pub fn new(role: Role) -> Creep {
        Creep {
            role,
            job: None,
        }
    }

    pub fn run(&self) {
        match self.job {
            Some(Job::Move{..}) => {}
            Some(Job::Harvest{..}) => {}
            _ => {}
        }
    }
}