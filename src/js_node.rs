use serde::{Deserialize, Serialize};

use crate::{
    metadata::{Metadata, WithMetadata},
    nodes::*,
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSNode {
    MultiplyNodeJS(MultiplyNodeJS),
    SumNodeJS(SumNodeJS),
    SineOscillatorNodeJS(SineOscillatorNodeJS),
    SawOscillatorNodeJS(SawOscillatorNodeJS),
    FloatSourceJS(Float32SourceJS),
    SplineFloatNode(SplineFloatNode)
}

impl WithMetadata for JSNode {
    fn metadata(&self) -> Metadata {
        match self {
            JSNode::MultiplyNodeJS(node) => node.metadata,
            JSNode::SumNodeJS(node) => node.metadata,
            JSNode::SineOscillatorNodeJS(node) => node.metadata,
            JSNode::FloatSourceJS(node) => node.metadata,
            JSNode::SawOscillatorNodeJS(node) => node.metadata,
            JSNode::SplineFloatNode(node) => node.metadata
        }
    }
}
