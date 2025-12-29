use serde::{Deserialize, Serialize};

use crate::metadata::Metadata;

#[derive(Serialize, Deserialize)]
pub struct MultiplyNodeJS {
    pub metadata: Metadata,
    pub multiplicand_source_id: usize,
    pub multiplier_source_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SumNodeJS {
    pub metadata: Metadata,
    pub augend_source_id: usize,
    pub addend_source_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SineOscillatorNodeJS {
    pub metadata: Metadata,
    pub sine_frequency_source_id: usize,

}

#[derive(Serialize, Deserialize)]
pub struct SawOscillatorNodeJS {
    pub metadata: Metadata,
    pub saw_frequency_source_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Float32SourceJS {
    pub metadata: Metadata,
    pub value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SplineFloatNode {
    pub metadata: Metadata,
    pub frequency_source_id: usize,
    pub points: Vec<Point>
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32
}