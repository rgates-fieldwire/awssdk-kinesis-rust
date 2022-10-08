#![forbid(unsafe_code)]
pub use json::{event_wrapper::EventWrapper, event::Event};

pub mod json;
pub mod actions;