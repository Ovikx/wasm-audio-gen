use serde::{Deserialize, Serialize};

use crate::JSNode;

#[derive(Deserialize, Serialize)]
pub struct ConfiguredSourceGraph {
    pub source_graph: SourceGraph,
    pub audio_context: AudioContext,
}

#[derive(Deserialize, Serialize)]
pub struct SourceGraph {
    pub root_id: usize,
    pub nodes: Vec<JSNode>,
}

#[derive(Deserialize, Serialize)]
pub struct AudioContext {
    pub sample_rate: f32,
}
