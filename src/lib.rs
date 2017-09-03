// Deny warnings inside doc tests / examples
#![doc(test(attr(deny(warnings))))]

extern crate libc;

mod ffi;
#[macro_use]
mod util;
mod error;
mod lua;
mod conversion;
mod multi;

#[cfg(test)]
mod tests;

pub use error::{Error, Result, ExternalError, ExternalResult};
pub use lua::{Value, Nil, ToLua, FromLua, MultiValue, ToLuaMulti, FromLuaMulti, Integer, Number,
              LightUserData, String, Table, TablePairs, TableSequence, Function, ThreadStatus,
              Thread, MetaMethod, UserDataMethods, UserData, AnyUserData, Lua};
pub use multi::Variadic;

pub mod prelude;
