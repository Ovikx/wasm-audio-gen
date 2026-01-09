use serde::{Deserialize, Serialize};

use crate::JSNode;

#[derive(Deserialize, Serialize)]
pub struct ConfiguredSourceGraph {
    pub nodes: Vec<JSNode>,
    pub audio_context: AudioContext,
}

#[derive(Deserialize, Serialize)]
pub struct AudioContext {
    pub sample_rate: f32,
}
