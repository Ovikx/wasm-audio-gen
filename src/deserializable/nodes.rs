use serde::{Deserialize, Serialize};

use crate::deserializable::metadata::Metadata;

#[derive(Serialize, Deserialize, Clone)]
pub struct MultiplyNodeJS {
    pub metadata: Metadata,
    pub multiplicand_source_id: usize,
    pub multiplier_source_id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SumNodeJS {
    pub metadata: Metadata,
    pub augend_source_id: usize,
    pub addend_source_id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SineOscillatorNodeJS {
    pub metadata: Metadata,
    pub sine_frequency_source_id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SawOscillatorNodeJS {
    pub metadata: Metadata,
    pub saw_frequency_source_id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SquareOscillatorNodeJS {
    pub metadata: Metadata,
    pub square_frequency_source_id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Float32SourceJS {
    pub metadata: Metadata,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SplineFloatNode {
    pub metadata: Metadata,
    pub frequency_source_id: usize,
    pub points: Vec<Point>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExternalFloatNode {
    pub metadata: Metadata,
    pub input_buffer_index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
