//! Grouse is a game.

#![feature(duration_float)]
#![allow(missing_docs)]
#![warn(missing_debug_implementations)]
#![forbid(unsafe_code, anonymous_parameters)]

mod graphics;
mod physics;
mod run;
mod terrain;
mod vector;
mod world;

pub use crate::{
    graphics::*,
    physics::*,
    run::*,
    terrain::*,
    vector::*,
    world::*,
};

#[allow(unused_imports)]
pub(crate) use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
