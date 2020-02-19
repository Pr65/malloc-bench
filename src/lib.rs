#![feature(weak_into_raw)]
#![feature(test)]
#![feature(allocator_api)]
#![feature(log_syntax)]

mod data_structures;
mod applications;
extern crate test;
include!("global.rs");


#[cfg(test)]
set_alloc!();

#[cfg(test)]
log_syntax!(setting malloc in test mode);