use std::sync::{Arc, Mutex};

use audio_gen::{
    input_buffer::SharedExternalInputBuffer,
    node::{
        ExternalFloatNode, FloatSource, MultiplyNode, SawOscillatorNode, SineOscillatorNode,
        SplineFloatNode, SquareOscillatorNode, SumNode,
    },
    source::Source,
};

use crate::{arc_mutex, deserializable::js_node::JSNode};

pub fn build_nodes(
    js_nodes: Vec<JSNode>,
    input_buffer: SharedExternalInputBuffer,
) -> Vec<Arc<Mutex<dyn Source>>> {
    js_nodes
        .iter()
        .map(|node: &JSNode| {
            let core_node: Arc<Mutex<dyn Source>> = match node {
                JSNode::MultiplyNodeJS(multiply_node_js) => arc_mutex!(MultiplyNode::new(
                    multiply_node_js.metadata.id,
                    multiply_node_js.multiplicand_source_id,
                    multiply_node_js.multiplier_source_id
                )),
                JSNode::SumNodeJS(sum_node_js) => arc_mutex!(SumNode::new(
                    sum_node_js.metadata.id,
                    sum_node_js.augend_source_id,
                    sum_node_js.addend_source_id
                )),
                JSNode::SineOscillatorNodeJS(sine_oscillator_node_js) => {
                    arc_mutex!(SineOscillatorNode::new(
                        sine_oscillator_node_js.metadata.id,
                        sine_oscillator_node_js.sine_frequency_source_id
                    ))
                }
                JSNode::SawOscillatorNodeJS(saw_oscillator_node_js) => {
                    arc_mutex!(SawOscillatorNode::new(
                        saw_oscillator_node_js.metadata.id,
                        saw_oscillator_node_js.saw_frequency_source_id
                    ))
                }
                JSNode::SquareOscillatorNodeJS(square_oscillator_node_js) => {
                    arc_mutex!(SquareOscillatorNode::new(
                        square_oscillator_node_js.metadata.id,
                        square_oscillator_node_js.square_frequency_source_id
                    ))
                }
                JSNode::FloatSourceJS(float_source_js) => arc_mutex!(FloatSource::new(
                    float_source_js.metadata.id,
                    float_source_js.value
                )),
                JSNode::SplineFloatNode(spline_float_node) => {
                    arc_mutex!(SplineFloatNode::new(
                        spline_float_node.metadata.id,
                        spline_float_node.frequency_source_id,
                        spline_float_node
                            .points
                            .iter()
                            .map(|point| (point.x, point.y))
                            .collect()
                    ))
                }
                JSNode::ExternalFloatNode(external_float_node) => {
                    arc_mutex!(ExternalFloatNode::new(
                        external_float_node.metadata.id,
                        input_buffer.clone(),
                        external_float_node.input_buffer_index
                    ))
                }
            };
            core_node
        })
        .collect()
}
