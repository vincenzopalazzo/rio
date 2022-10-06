//! Toy library to implement an async library for rust!
//!
//! This code is hevely based on https://github.com/mgattozzi/whorl but with
//! another scope in mind, that is to provide a toy library that the people can
//! use with toy program.
//!
//! In addition, from this crate the user can consider to learn more on the async
//! programming, because there is a lot to learn and a lot to contribute.
// TODO: move feature inside a std :)
#![feature(once_cell)]
pub mod lazy;
pub mod runitime;
pub mod spawner;
pub mod task;
