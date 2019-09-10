#![allow(unused_variables)]
use screeps::{Position, ResourceType};
use stdweb::{__js_serializable_boilerplate, js_serializable, js_deserializable};
use super::Creep;

type Id = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    GoTo { pos: Position },
    GoToRoom { room_id: Id },
    TransferAll { target: Id, resource: ResourceType },
    TransferAmount { target: Id, resource: ResourceType, amount: u32 },
    WithdrawAll { target: Id, resource: ResourceType },
    WithdrawAmount { target: Id, resource: ResourceType, amount: u32 },
    PickupAll { target: Id },
    PickupAmount { target: Id, amount: u32 },
    Harvest { target: Id },
    Build { site: Id },
    Dismantle { target: Id },
    Repair { target: Id },
    Fortify { target: Id },
    ControllerAttack { controller: Id },
    ControllerClaim { controller: Id },
    ControllerUpgrade { controller: Id },
    ControllerReserve { controller: Id },
    Heal { target: Id },
    HealRanged { target: Id },
    AttackMelee { target: Id },
    AttackRanged { target: Id },
    AttackRangedMass,
    GetBoosted { lab: Id },
    GetRenewed { spawn: Id },
}
js_serializable!(Action);
js_deserializable!(Action);

pub fn go_to(creep: &mut Creep, pos: Position) {
    // Go to `pos`
    // When done, remove the action
}

pub fn go_to_room(creep: &mut Creep, room_id: Id) {

}

pub fn transfer_all(creep: &mut Creep, target: Id, resource: ResourceType) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn transfer_amount(creep: &mut Creep, target: Id, resource: ResourceType, amount: u32) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn withdraw_all(creep: &mut Creep, target: Id, resource: ResourceType) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn withdraw_amount(creep: &mut Creep, target: Id, resource: ResourceType, amount: u32) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn pickup_all(creep: &mut Creep, target: Id) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn pickup_amount(creep: &mut Creep, target: Id, amount: u32) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn harvest(creep: &mut Creep, target: Id) {
    // If target is not near, order a GoTo first
}

pub fn build(creep: &mut Creep, site: Id) {
    // If target is not near, order a GoTo first
}

pub fn dismantle(creep: &mut Creep, target: Id) {
    // If target is not near, order a GoTo first
}

pub fn repair(creep: &mut Creep, target: Id) {
    // If target is not near, order a GoTo first
}

pub fn fortify(creep: &mut Creep, target: Id) {
    // If target is not near, order a GoTo first
}

pub fn controller_attack(creep: &mut Creep, controller: Id) {
    // If target is not near, order a GoTo first
}

pub fn controller_claim(creep: &mut Creep, controller: Id) {
    // If target is not near, order a GoTo first
}

pub fn controller_upgrade(creep: &mut Creep, controller: Id) {
    // If target is not near, order a GoTo first
}

pub fn controller_reserve(creep: &mut Creep, controller: Id) {
    // If target is not near, order a GoTo first
}

pub fn heal(creep: &mut Creep, target: Id) {
}

pub fn heal_ranged(creep: &mut Creep, target: Id) {
}

pub fn attack_melee(creep: &mut Creep, target: Id) {
}

pub fn attack_ranged(creep: &mut Creep, target: Id) {
}

pub fn attack_ranged_mass(creep: &mut Creep) {
}

pub fn get_boosted(creep: &mut Creep, lab: Id) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}

pub fn get_renewed(creep: &mut Creep, spawn: Id) {
    // If target is not near, order a GoTo first
    // When done, remove the action
}