#![crate_type = "lib"]
#![crate_name = "portaudio"]
#![feature(phase, unsafe_destructor)]

extern crate libc;
#[phase(plugin, link)] extern crate log;

pub mod stream;
pub mod pa;

mod ll;
mod util;
