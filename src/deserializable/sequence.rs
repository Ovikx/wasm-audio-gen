use serde::{Deserialize, Serialize};

use crate::deserializable::{js_node::JSNode, source_graph::AudioContext};

#[derive(Deserialize, Serialize)]
pub struct GeneratorSequence {
    pub audio_context: AudioContext,
    pub generator_intervals: Vec<GeneratorInterval>,
}

#[derive(Deserialize, Serialize)]
pub struct GeneratorInterval {
    pub nodes: Vec<JSNode>,
    pub start_sample: u32,
    pub end_sample: u32,
}
