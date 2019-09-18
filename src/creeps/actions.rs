#![allow(unused_variables)]
use super::Creep;
use crate::error::Res;
use screeps::{
    game::get_object_erased, traits::IntoExpectedType, ConstructionSite, HasPosition,
    MoveToOptions, Position, ResourceType, Source, Structure, StructureController, StructureLab, StructureSpawn,
};
use stdweb::{Reference, __js_serializable_boilerplate, js_deserializable, js_serializable};

type Id = String;

const RANGE_DISMANTLE: u32 = 1;
const RANGE_TRANSFER: u32 = 1;
const RANGE_WITHDRAW: u32 = 1;
const RANGE_HARVEST: u32 = 1;
const RANGE_BUILD: u32 = 3;
const RANGE_REPAIR: u32 = 3;
const RANGE_ATTACK_MELEE: u32 = 1;
const RANGE_ATTACK_RANGED: u32 = 3;
const RANGE_CONTROLLER_ATTACK: u32 = 1;
const RANGE_CONTROLLER_CLAIM: u32 = 1;
const RANGE_CONTROLLER_RESERVE: u32 = 1;
const RANGE_CONTROLLER_UPGRADE: u32 = 3;
const RANGE_BOOST: u32 = 1;
const RANGE_RENEW: u32 = 1;
const RANGE_RECYCLE: u32 = 1;

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
        target_id: Id,
        resource: ResourceType,
    },
    TransferAmount {
        target_id: Id,
        resource: ResourceType,
        amount: u32,
    },
    WithdrawAll {
        target_id: Id,
        resource: ResourceType,
    },
    WithdrawAmount {
        target_id: Id,
        resource: ResourceType,
        amount: u32,
    },
    PickupAll {
        target_id: Id,
    },
    PickupAmount {
        target_id: Id,
        amount: u32,
    },
    Harvest {
        target_id: Id,
    },
    Build {
        site_id: Id,
    },
    Dismantle {
        target_id: Id,
    },
    Repair {
        target_id: Id,
    },
    Fortify {
        target_id: Id,
    },
    ControllerAttack {
        controller_id: Id,
    },
    ControllerClaim {
        controller_id: Id,
    },
    ControllerUpgrade {
        controller_id: Id,
    },
    ControllerReserve {
        controller_id: Id,
    },
    Heal {
        creep_id: Id,
    },
    HealRanged {
        creep_id: Id,
    },
    AttackMelee {
        creep_id: Id,
    },
    AttackRanged {
        creep_id: Id,
    },
    AttackRangedMass,
    GetBoosted {
        lab_id: Id,
    },
    GetRenewed {
        spawn_id: Id,
    },
    GetRecycled {
        spawn_id: Id,
    },
}

js_serializable!(Action);
js_deserializable!(Action);

impl Action {
    pub fn run(&self, creep: &mut Creep) -> Res<()> {
        use Action::*;
        match self {
            GoTo { pos } => {
                go_to(creep, *pos)?;
            }
            GoToRoom { room_id } => {
                go_to_room(creep, room_id)?;
            }
            GoToRanged { pos, range } => {
                go_to_ranged(creep, *pos, *range)?;
            }
            TransferAll {
                target_id,
                resource,
            } => {
                transfer_all(creep, target_id, resource)?;
            }
            TransferAmount {
                target_id,
                resource,
                amount,
            } => {
                transfer_amount(creep, target_id, resource, *amount)?;
            }
            WithdrawAll {
                target_id,
                resource,
            } => {
                withdraw_all(creep, target_id, resource)?;
            }
            WithdrawAmount {
                target_id,
                resource,
                amount,
            } => {
                withdraw_amount(creep, target_id, resource, *amount)?;
            }
            PickupAll { target_id } => {
                pickup_all(creep, target_id)?;
            }
            PickupAmount { target_id, amount } => {
                pickup_amount(creep, target_id, *amount)?;
            }
            Harvest { target_id } => {
                harvest(creep, target_id)?;
            }
            Build { site_id } => {
                build(creep, site_id)?;
            }
            Dismantle { target_id } => {
                dismantle(creep, target_id)?;
            }
            Repair { target_id } => {
                repair(creep, target_id)?;
            }
            Fortify { target_id } => {
                fortify(creep, target_id)?;
            }
            ControllerAttack { controller_id } => {
                controller_attack(creep, controller_id)?;
            }
            ControllerClaim { controller_id } => {
                controller_claim(creep, controller_id)?;
            }
            ControllerUpgrade { controller_id } => {
                controller_upgrade(creep, controller_id)?;
            }
            ControllerReserve { controller_id } => {
                controller_reserve(creep, controller_id)?;
            }
            Heal { creep_id } => {
                heal(creep, creep_id)?;
            }
            HealRanged { creep_id } => {
                heal_ranged(creep, creep_id)?;
            }
            AttackMelee { creep_id } => {
                attack_melee(creep, creep_id)?;
            }
            AttackRanged { creep_id } => {
                attack_ranged(creep, creep_id)?;
            }
            AttackRangedMass => {
                attack_ranged_mass(creep)?;
            }
            GetBoosted { lab_id } => {
                get_boosted(creep, lab_id)?;
            }
            GetRenewed { spawn_id } => {
                get_renewed(creep, spawn_id)?;
            }
            GetRecycled { spawn_id } => {
                get_recycled(creep, spawn_id)?;
            }
        }
        Ok(())
    }
}

fn prepend_go_to_if_far(creep: &mut Creep, pos: Position, range: u32) -> Res<()> {
    // If target is not in range, order a GoTo first
    if !creep.obj.pos().in_range_to(&pos, range) {
        creep
            .memory
            .actions
            .push_front(Action::GoToRanged { pos, range });
        // and do it
        go_to_ranged(creep, pos, range)?;
    }

    Ok(())
}

pub fn go_to(creep: &mut Creep, pos: Position) -> Res<()> {
    creep.obj.move_to(&pos);

    // When done, remove the action
    if creep.obj.pos().is_equal_to(&pos) {
        creep.memory.actions.pop_front();
    }

    // FIXME: Check if it worked

    Ok(())
}

pub fn go_to_room(creep: &mut Creep, room_id: &Id) -> Res<()> {
    unimplemented!()
}

pub fn go_to_ranged(creep: &mut Creep, pos: Position, range: u32) -> Res<()> {
    let options = MoveToOptions::new().range(range);
    creep.obj.move_to_with_options(&pos, options);

    // When done, remove the action
    if creep.obj.pos().in_range_to(&pos, range) {
        creep.memory.actions.pop_front();
    }

    // FIXME: Check if it worked

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
    let target: Source = screeps::game::get_object_typed(&target_id)?
        .ok_or_else(|| format!("no object with id {}", target_id))?;

    prepend_go_to_if_far(creep, target.pos(), RANGE_HARVEST)?;
    creep.obj.harvest(&target);
    
    // FIXME: Check if it worked

    Ok(())
}

pub fn build(creep: &mut Creep, site_id: &Id) -> Res<()> {
    let site: ConstructionSite = screeps::game::get_object_typed(&site_id)?
        .ok_or_else(|| format!("no object with id {}", site_id))?;

    prepend_go_to_if_far(creep, site.pos(), RANGE_BUILD)?;
    creep.obj.build(&site);

    // FIXME: Check if it worked
    // FIXME: If not enough energy, get more energy
    // FIXME: If done, remove job

    Ok(())
}

pub fn dismantle(creep: &mut Creep, target_id: &Id) -> Res<()> {
    let target = get_structure_from_id(target_id)?;

    prepend_go_to_if_far(creep, target.pos(), RANGE_DISMANTLE)?;
    creep.obj.dismantle(&target);

    // FIXME: Check if it worked
    // FIXME: If done, remove job

    Ok(())
}

pub fn repair(creep: &mut Creep, target_id: &Id) -> Res<()> {
    let target = get_structure_from_id(target_id)?;

    prepend_go_to_if_far(creep, target.pos(), RANGE_REPAIR)?;
    creep.obj.repair(&target);

    // FIXME: Check if it worked
    // FIXME: If not enough energy, get more energy
    // FIXME: If done, remove job

    Ok(())
}

pub fn fortify(creep: &mut Creep, target_id: &Id) -> Res<()> {
    let target = get_structure_from_id(target_id)?;

    prepend_go_to_if_far(creep, target.pos(), RANGE_REPAIR)?;
    creep.obj.repair(&target);

    // FIXME: Check if it worked
    // FIXME: If not enough energy, get more energy
    // FIXME: Do for number of ticks?
    // FIXME: If done, remove job

    Ok(())
}

pub fn controller_attack(creep: &mut Creep, controller_id: &Id) -> Res<()> {
    let controller: StructureController = screeps::game::get_object_typed(&controller_id)?
        .ok_or_else(|| format!("no object with id {}", controller_id))?;

    prepend_go_to_if_far(creep, controller.pos(), RANGE_CONTROLLER_ATTACK)?;
    creep.obj.attack_controller(&controller);

    // FIXME: Check if it worked
    // FIXME: If done, remove job

    Ok(())
}

pub fn controller_claim(creep: &mut Creep, controller_id: &Id) -> Res<()> {
    let controller: StructureController = screeps::game::get_object_typed(&controller_id)?
        .ok_or_else(|| format!("no object with id {}", controller_id))?;

    prepend_go_to_if_far(creep, controller.pos(), RANGE_CONTROLLER_CLAIM)?;
    creep.obj.claim_controller(&controller);

    // FIXME: Check if it worked
    // FIXME: If done, remove job

    Ok(())
}

pub fn controller_upgrade(creep: &mut Creep, controller_id: &Id) -> Res<()> {
    let controller: StructureController = screeps::game::get_object_typed(&controller_id)?
        .ok_or_else(|| format!("no object with id {}", controller_id))?;

    prepend_go_to_if_far(creep, controller.pos(), RANGE_CONTROLLER_UPGRADE)?;
    creep.obj.upgrade_controller(&controller);

    // FIXME: Check if it worked
    // FIXME: If not enough energy, get more energy
    // FIXME: If done, remove job

    Ok(())
}

pub fn controller_reserve(creep: &mut Creep, controller_id: &Id) -> Res<()> {
    let controller: StructureController = screeps::game::get_object_typed(&controller_id)?
        .ok_or_else(|| format!("no object with id {}", controller_id))?;

    prepend_go_to_if_far(creep, controller.pos(), RANGE_CONTROLLER_RESERVE)?;
    creep.obj.reserve_controller(&controller);

    // FIXME: Check if it worked
    // TODO: stop at some point?

    Ok(())
}

pub fn heal(creep: &mut Creep, creep_id: &Id) -> Res<()> {
    unimplemented!()
}

pub fn heal_ranged(creep: &mut Creep, creep_id: &Id) -> Res<()> {
    unimplemented!()
}

pub fn attack_melee(creep: &mut Creep, creep_id: &Id) -> Res<()> {
    unimplemented!()
}

pub fn attack_ranged(creep: &mut Creep, creep_id: &Id) -> Res<()> {
    unimplemented!()
}

pub fn attack_ranged_mass(creep: &mut Creep) -> Res<()> {
    unimplemented!()
}

pub fn get_boosted(creep: &mut Creep, lab_id: &Id) -> Res<()> {
    let lab: StructureLab = screeps::game::get_object_typed(&lab_id)?
        .ok_or_else(|| format!("no object with id {}", lab_id))?;

    prepend_go_to_if_far(creep, lab.pos(), RANGE_BOOST)?;
    lab.boost_creep(&creep.obj, None);

    // FIXME: Check if it worked
    // FIXME: If done, remove job

    Ok(())
}

pub fn get_renewed(creep: &mut Creep, spawn_id: &Id) -> Res<()> {
    let spawn: StructureSpawn = screeps::game::get_object_typed(&spawn_id)?
        .ok_or_else(|| format!("no object with id {}", spawn_id))?;

    prepend_go_to_if_far(creep, spawn.pos(), RANGE_RENEW)?;
    spawn.renew_creep(&creep.obj);

    // FIXME: Check if it worked
    // FIXME: If done, remove job

    Ok(())
}

pub fn get_recycled(creep: &mut Creep, spawn_id: &Id) -> Res<()> {
    let spawn: StructureSpawn = screeps::game::get_object_typed(&spawn_id)?
        .ok_or_else(|| format!("no object with id {}", spawn_id))?;

    prepend_go_to_if_far(creep, spawn.pos(), RANGE_RECYCLE)?;
    spawn.recycle_creep(&creep.obj);

    // FIXME: Check if it worked
    // FIXME: If done, remove job

    Ok(())
}

fn get_structure_from_id(id: &Id) -> Res<Structure> {
    let reference: Reference = get_object_erased(id)
        .ok_or_else(|| format!("no object with id {}", id))?
        .into();
    Ok(reference.into_expected_type()?)
}
