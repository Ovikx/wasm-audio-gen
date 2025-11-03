use serde::{Deserialize, Serialize};

use crate::metadata::Metadata;

#[derive(Serialize, Deserialize)]
pub struct MultiplyNodeJS {
    pub metadata: Metadata,
    pub multiplicand_source_id: i32,
    pub multiplier_source_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SumNodeJS {
    pub metadata: Metadata,
    pub augend_source_id: i32,
    pub addend_source_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SineOscillatorNodeJS {
    pub metadata: Metadata,
    pub frequency_source_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Float32SourceJS {
    pub metadata: Metadata,
    pub value: f32,
}
