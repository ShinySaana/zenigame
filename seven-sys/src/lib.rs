#![no_std]
#![feature(concat_idents)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[path="bindgen_bindings.rs"]
pub mod bindings;
pub mod addresses;
pub mod bitfields;
pub mod functions;
