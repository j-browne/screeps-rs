use std::fmt;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoleType {
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

js_serializable!(RoleType);
js_deserializable!(RoleType);

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleType::Harvester => write!(f, "H"),
            RoleType::Transporter => write!(f, "T"),
            RoleType::Builder => write!(f, "B"),
            RoleType::Upgrader => write!(f, "U"),
            RoleType::Attacker => write!(f, "A"),
        }
    }
}
