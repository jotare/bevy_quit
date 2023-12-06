#![allow(clippy::needless_doctest_main)]
//! `bevy_quit` is a simple bevy plugin to easily add keybindings to exit a
//! [bevy](https://docs.rs/bevy/latest/bevy/) game.
//!
//! ## Example
//!
//! ```
//! use bevy::prelude::*;
//!
//! use bevy_quit::QuitPlugin;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(
//!             QuitPlugin::default() // default will add C-q
//!                 .add_key_binding(KeyCode::Escape)
//!                 .add_key_binding((KeyCode::ControlLeft, KeyCode::W))
//!                 .add_key_binding(vec![
//!                     KeyCode::ControlLeft,
//!                     KeyCode::ShiftLeft,
//!                     KeyCode::AltLeft,
//!                     KeyCode::C,
//!                 ]),
//!         )
//!         .run();
//! }
//! ```
//!

mod keybinding;
mod plugin;

pub use keybinding::KeyBinding;
pub use plugin::QuitPlugin;
