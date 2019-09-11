#![allow(unused_variables)]
use super::Creep;
use crate::error::Res;
use screeps::{HasPosition, MoveToOptions, Position, ResourceType};
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};

type Id = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// TODO: GoToWithOptions? For avoiding certain spots?
pub enum Action {
    GoTo {
        pos: Position,
    },
    GoToRoom {
        room_id: Id,
    },
    GoToRanged {
        pos: Position,
        range: u32,
    },
    TransferAll {
        target: Id,
        resource: ResourceType,
    },
    TransferAmount {
        target: Id,
        resource: ResourceType,
        amount: u32,
    },
    WithdrawAll {
        target: Id,
        resource: ResourceType,
    },
    WithdrawAmount {
        target: Id,
        resource: ResourceType,
        amount: u32,
    },
    PickupAll {
        target: Id,
    },
    PickupAmount {
        target: Id,
        amount: u32,
    },
    Harvest {
        target: Id,
    },
    Build {
        site: Id,
    },
    Dismantle {
        target: Id,
    },
    Repair {
        target: Id,
    },
    Fortify {
        target: Id,
    },
    ControllerAttack {
        controller: Id,
    },
    ControllerClaim {
        controller: Id,
    },
    ControllerUpgrade {
        controller: Id,
    },
    ControllerReserve {
        controller: Id,
    },
    Heal {
        target: Id,
    },
    HealRanged {
        target: Id,
    },
    AttackMelee {
        target: Id,
    },
    AttackRanged {
        target: Id,
    },
    AttackRangedMass,
    GetBoosted {
        lab: Id,
    },
    GetRenewed {
        spawn: Id,
    },
}

js_serializable!(Action);
js_deserializable!(Action);

impl Action {
    pub fn run(&self, creep: &mut Creep) -> Res<()> {
        use Action::*;
        match self {
            GoTo { pos } => {
                go_to(creep, pos)?;
            }
            GoToRoom { room_id } => {
                go_to_room(creep, room_id)?;
            }
            GoToRanged { pos, range } => {
                go_to_ranged(creep, pos, *range)?;
            }
            TransferAll { target, resource } => {
                transfer_all(creep, target, resource)?;
            }
            TransferAmount {
                target,
                resource,
                amount,
            } => {
                transfer_amount(creep, target, resource, *amount)?;
            }
            WithdrawAll { target, resource } => {
                withdraw_all(creep, target, resource)?;
            }
            WithdrawAmount {
                target,
                resource,
                amount,
            } => {
                withdraw_amount(creep, target, resource, *amount)?;
            }
            PickupAll { target } => {
                pickup_all(creep, target)?;
            }
            PickupAmount { target, amount } => {
                pickup_amount(creep, target, *amount)?;
            }
            Harvest { target } => {
                harvest(creep, target)?;
            }
            Build { site } => {
                build(creep, site)?;
            }
            Dismantle { target } => {
                dismantle(creep, target)?;
            }
            Repair { target } => {
                repair(creep, target)?;
            }
            Fortify { target } => {
                fortify(creep, target)?;
            }
            ControllerAttack { controller } => {
                controller_attack(creep, controller)?;
            }
            ControllerClaim { controller } => {
                controller_claim(creep, controller)?;
            }
            ControllerUpgrade { controller } => {
                controller_upgrade(creep, controller)?;
            }
            ControllerReserve { controller } => {
                controller_reserve(creep, controller)?;
            }
            Heal { target } => {
                heal(creep, target)?;
            }
            HealRanged { target } => {
                heal_ranged(creep, target)?;
            }
            AttackMelee { target } => {
                attack_melee(creep, target)?;
            }
            AttackRanged { target } => {
                attack_ranged(creep, target)?;
            }
            AttackRangedMass => {
                attack_ranged_mass(creep)?;
            }
            GetBoosted { lab } => {
                get_boosted(creep, lab)?;
            }
            GetRenewed { spawn } => {
                get_renewed(creep, spawn)?;
            }
        }
        Ok(())
    }
}

pub fn go_to(creep: &mut Creep, pos: &Position) -> Res<()> {
    // Go to `pos`
    creep.obj.move_to(pos);

    // When done, remove the action
    if creep.obj.pos().is_equal_to(pos) {
        creep.memory.actions.pop_front();
    }

    Ok(())
}

pub fn go_to_room(creep: &mut Creep, room_id: &Id) -> Res<()> {
    unimplemented!()
}

pub fn go_to_ranged(creep: &mut Creep, pos: &Position, range: u32) -> Res<()> {
    // Go to `pos`
    let options = MoveToOptions::new().range(range);
    creep.obj.move_to_with_options(pos, options);

    // When done, remove the action
    if creep.obj.pos().in_range_to(pos, range) {
        creep.memory.actions.pop_front();
    }

    Ok(())
}

pub fn transfer_all(creep: &mut Creep, target: &Id, resource: &ResourceType) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn transfer_amount(
    creep: &mut Creep,
    target: &Id,
    resource: &ResourceType,
    amount: u32,
) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn withdraw_all(creep: &mut Creep, target: &Id, resource: &ResourceType) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn withdraw_amount(
    creep: &mut Creep,
    target: &Id,
    resource: &ResourceType,
    amount: u32,
) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn pickup_all(creep: &mut Creep, target: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn pickup_amount(creep: &mut Creep, target: &Id, amount: u32) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn harvest(creep: &mut Creep, target_id: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    let target = screeps::game::get_object_erased(&target_id)
        .ok_or_else(|| format!("no object with id {}", target_id))?;
    let target_pos = target.pos();
    if !creep.obj.pos().is_near_to(&target_pos) {
        creep.memory.actions.push_front(Action::GoToRanged {
            pos: target_pos,
            range: 1,
        });
    }

    Ok(())
}

pub fn build(creep: &mut Creep, site: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn dismantle(creep: &mut Creep, target: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn repair(creep: &mut Creep, target: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn fortify(creep: &mut Creep, target: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn controller_attack(creep: &mut Creep, controller: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn controller_claim(creep: &mut Creep, controller: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn controller_upgrade(creep: &mut Creep, controller: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn controller_reserve(creep: &mut Creep, controller: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    unimplemented!()
}

pub fn heal(creep: &mut Creep, target: &Id) -> Res<()> {
    unimplemented!()
}

pub fn heal_ranged(creep: &mut Creep, target: &Id) -> Res<()> {
    unimplemented!()
}

pub fn attack_melee(creep: &mut Creep, target: &Id) -> Res<()> {
    unimplemented!()
}

pub fn attack_ranged(creep: &mut Creep, target: &Id) -> Res<()> {
    unimplemented!()
}

pub fn attack_ranged_mass(creep: &mut Creep) -> Res<()> {
    unimplemented!()
}

pub fn get_boosted(creep: &mut Creep, lab: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}

pub fn get_renewed(creep: &mut Creep, spawn: &Id) -> Res<()> {
    // If target is not near, order a GoTo first
    // When done, remove the action
    unimplemented!()
}
