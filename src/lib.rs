use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::deserializable::js_node::JSNode;

extern crate console_error_panic_hook;
extern crate web_sys;

mod build_nodes;
mod deserializable;
mod generator;
mod macros;
mod sequencer;

pub use generator::SampleGenerator;
pub use sequencer::Sequencer;

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
