use serde::{Deserialize, Serialize};

use crate::deserializable::nodes::*;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSNode {
    MultiplyNodeJS(MultiplyNodeJS),
    SumNodeJS(SumNodeJS),
    SineOscillatorNodeJS(SineOscillatorNodeJS),
    SawOscillatorNodeJS(SawOscillatorNodeJS),
    SquareOscillatorNodeJS(SquareOscillatorNodeJS),
    FloatSourceJS(Float32SourceJS),
    SplineFloatNode(SplineFloatNode),
}
