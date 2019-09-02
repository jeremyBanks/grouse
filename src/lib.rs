//! Grouse is a game.

#![feature(duration_float, impl_trait_in_bindings)]
#![allow(missing_docs)]
#![warn(missing_debug_implementations, unused_results)]
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

pub(crate) use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
