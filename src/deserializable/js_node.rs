use serde::{Deserialize, Serialize};

use crate::deserializable::nodes::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum JSNode {
    MultiplyNodeJS(MultiplyNodeJS),
    SumNodeJS(SumNodeJS),
    SineOscillatorNodeJS(SineOscillatorNodeJS),
    SawOscillatorNodeJS(SawOscillatorNodeJS),
    SquareOscillatorNodeJS(SquareOscillatorNodeJS),
    FloatSourceJS(Float32SourceJS),
    SplineFloatNode(SplineFloatNode),
    ExternalFloatNode(ExternalFloatNode),
}
