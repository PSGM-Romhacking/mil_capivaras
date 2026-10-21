//! Casio PV-1000 core, reimplemented from the MiSTer core's Verilog.
//! It does not depend on a frontend: it serves Mil Capivaras and the future libretro core.

pub mod core;
pub mod machine;
pub mod games;
pub mod memory;

pub use core::{INFO, Pv1000Core};
