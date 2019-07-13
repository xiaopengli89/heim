//! Host system information.
//!
//! This module is enabled with the `host` feature flag (enabled by default).

#![deny(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]

#[cfg(target_os = "windows")]
#[macro_use]
extern crate winapi;

pub use platforms::target::Arch;

mod sys;

pub mod os;
mod platform;
mod uptime;
mod users;

pub use self::platform::*;
pub use self::uptime::*;
pub use self::users::*;

#[cfg(any(doc, not(target_os = "windows")))]
type Pid = libc::pid_t;

pub use heim_common::units::Time;
