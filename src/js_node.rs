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
    Float32SourceJS(Float32SourceJS),
}

impl WithMetadata for JSNode {
    fn metadata(&self) -> Metadata {
        match self {
            JSNode::MultiplyNodeJS(node) => node.metadata,
            JSNode::SumNodeJS(node) => node.metadata,
            JSNode::SineOscillatorNodeJS(node) => node.metadata,
            JSNode::Float32SourceJS(node) => node.metadata,
        }
    }
}
