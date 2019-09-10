use crate::error::Res;
use self::actions::Action;
use screeps::Creep as ScreepCreep;
use std::{
    collections::VecDeque,
    fmt,
};
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

mod actions;

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
        screeps::memory::root().path_set(&format!("creeps.{}", self.obj.name()), self.memory.clone());
    }
}

impl Creep {
    pub fn new(obj: ScreepCreep) -> Res<Self> {
        let memory = screeps::memory::root().get_path(&format!("creeps.{}", obj.name()))?
            .ok_or_else(|| format!("undefined or null creep memory for {}", obj.name()))?;
        Ok(Self {
            obj,
            memory,
        })
    }

    pub fn run(&mut self) -> Res<()> {
        let actions = &mut self.memory.actions;
        match actions.front().cloned() {
            Some(Action::GoTo { pos }) => {
                actions::go_to(self, pos);
            }
            Some(Action::GoToRoom { room_id }) => {
                actions::go_to_room(self, room_id);
            }
            Some(Action::TransferAll { target, resource }) => {
                actions::transfer_all(self, target, resource);
            }
            Some(Action::TransferAmount { target, resource, amount }) => {
                actions::transfer_amount(self, target, resource, amount);
            }
            Some(Action::WithdrawAll { target, resource }) => {
                actions::withdraw_all(self, target, resource);
            }
            Some(Action::WithdrawAmount { target, resource, amount }) => {
                actions::withdraw_amount(self, target, resource, amount);
            }
            Some(Action::PickupAll { target }) => {
                actions::pickup_all(self, target);
            }
            Some(Action::PickupAmount { target, amount }) => {
                actions::pickup_amount(self, target, amount);
            }
            Some(Action::Harvest { target }) => {
                actions::harvest(self, target);
            }
            Some(Action::Build { site }) => {
                actions::build(self, site);
            }
            Some(Action::Dismantle { target }) => {
                actions::dismantle(self, target);
            }
            Some(Action::Repair { target }) => {
                actions::repair(self, target);
            }
            Some(Action::Fortify { target }) => {
                actions::fortify(self, target);
            }
            Some(Action::ControllerAttack { controller }) => {
                actions::controller_attack(self, controller);
            }
            Some(Action::ControllerClaim { controller }) => {
                actions::controller_claim(self, controller);
            }
            Some(Action::ControllerUpgrade { controller }) => {
                actions::controller_upgrade(self, controller);
            }
            Some(Action::ControllerReserve { controller }) => {
                actions::controller_reserve(self, controller);
            }
            Some(Action::Heal { target }) => {
                actions::heal(self, target);
            }
            Some(Action::HealRanged { target }) => {
                actions::heal_ranged(self, target);
            }
            Some(Action::AttackMelee { target }) => {
                actions::attack_melee(self, target);
            }
            Some(Action::AttackRanged { target }) => {
                actions::attack_ranged(self, target);
            }
            Some(Action::AttackRangedMass) => {
                actions::attack_ranged_mass(self);
            }
            Some(Action::GetBoosted { lab }) => {
                actions::get_boosted(self, lab);
            }
            Some(Action::GetRenewed { spawn }) => {
                actions::get_renewed(self, spawn);
            }
            None => {}
        }

        Ok(())
    }
}
