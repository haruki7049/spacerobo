//! On-board computer: a Bevy-independent RV32I virtual machine.
//!
//! Players will eventually program their robo's in-game behavior directly, by writing RISC-V
//! machine code that this crate executes. This crate depends on no other workspace crate and
//! not on `bevy`/`avian3d`, so it can be developed and tested without the game engine.

pub mod bus;
pub mod cpu;

pub use bus::{Bus, Mmio, Ram};
pub use cpu::Cpu;
