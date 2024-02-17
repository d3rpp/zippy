#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R>() -> TauriPlugin<R>
where
    R: Runtime,
{
    Builder::<R, _>::new("zippy").build()
}
