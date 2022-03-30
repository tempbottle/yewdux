#![doc = include_str!("../../../README.md")]
#![allow(clippy::needless_doctest_main)]

// pub mod component;
mod context;
pub mod dispatch;
pub mod functional;
pub mod store;
mod util;

pub mod prelude {
    //! Everything you need to use Yewdux.

    pub use crate::{
        dispatch::{self, Dispatch},
        functional::use_store,
        store::{Message, Store},
    };
}