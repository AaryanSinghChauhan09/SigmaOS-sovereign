//! SigmaOS-sovereign — Sovereign init (PID 1) and service supervisor.
//!
//! Extracted from the SigmaOS-system monolith. Contains the versioned
//! init/supervisor design suite (v52–v76 + PR snapshots) and the base
//! `sovereign_init_supervisor` implementation.
#![no_std]

#[macro_use]
extern crate alloc;

pub mod sovereign_init_supervisor;
pub mod sovereign_init_and_supervisor_inspirations_pr;
pub mod sovereign_init_and_supervisor_advancements_v52;
pub mod sovereign_init_and_supervisor_advancements_v53;
pub mod sovereign_init_and_supervisor_advancements_v60;
pub mod sovereign_init_and_supervisor_advancements_v61;
pub mod sovereign_init_and_supervisor_advancements_v62;
pub mod sovereign_init_and_supervisor_advancements_v63;
pub mod sovereign_init_and_supervisor_advancements_v63_pr;
pub mod sovereign_init_and_supervisor_advancements_v67;
pub mod sovereign_init_and_supervisor_advancements_v68_pr;
pub mod sovereign_init_and_supervisor_advancements_v69_pr;
pub mod sovereign_init_and_supervisor_advancements_v71_pr;
pub mod sovereign_init_and_supervisor_advancements_v75_pr;
pub mod sovereign_init_and_supervisor_advancements_v76_pr;

/// Module extracted from the SigmaOS-system monolith.
pub mod abi;
