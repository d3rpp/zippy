//! this is the tauri plugin for the [`zippy-core`](zippy_core) crate, it is early stages so there will be more to come

#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

///
#[must_use]
pub fn init<R>() -> TauriPlugin<R>
where
    R: Runtime,
{
    Builder::<R, _>::new("zippy").build()
}
