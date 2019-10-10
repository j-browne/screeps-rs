#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod controllers;
pub mod creeps;
pub mod error;
pub mod logging;
pub mod mayor;
pub mod names;
pub mod rooms;

pub type Id = String;
