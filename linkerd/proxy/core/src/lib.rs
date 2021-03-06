#![deny(warnings, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(clippy::inconsistent_struct_constructor)]

pub mod resolve;

pub use self::resolve::{Resolve, ResolveService, Update};
