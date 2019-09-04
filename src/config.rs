use crate::roles::RoleType;
use screeps::constants::Part;
use std::collections::HashMap;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub roles_to_spawn: HashMap<String, Vec<RoleType>>,
    pub equip: HashMap<RoleType, Vec<Part>>,
}

js_serializable!(Config);
js_deserializable!(Config);