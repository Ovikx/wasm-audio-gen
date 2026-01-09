use serde::{Deserialize, Serialize};

use crate::deserializable::source_graph::{AudioContext, SourceGraph};

#[derive(Deserialize, Serialize)]
pub struct GeneratorSequence {
    pub audio_context: AudioContext,
    pub generator_intervals: Vec<GeneratorInterval>,
}

#[derive(Deserialize, Serialize)]
pub struct GeneratorInterval {
    pub source_graph: SourceGraph,
    pub start_sample: u32,
    pub end_sample: u32,
}
