use std::{cell::RefCell, rc::Rc};

use audio_gen::{
    context::AudioContext,
    generator::SampleGenerator,
    node::{
        FloatSource, MultiplyNode, SawOscillatorNode, SineOscillatorNode, SplineFloatNode,
        SquareOscillatorNode, SumNode,
    },
    source::Source,
};
use wasm_bindgen::JsValue;

use crate::{
    deserializable::{js_node::JSNode, source_graph::ConfiguredSourceGraph},
    rc_refcell,
};

pub fn build_generator(js_graph: JsValue) -> SampleGenerator {
    let graph: ConfiguredSourceGraph = serde_wasm_bindgen::from_value(js_graph).unwrap();
    let nodes: Vec<Rc<RefCell<dyn Source>>> = graph
        .source_graph
        .nodes
        .iter()
        .map(|node: &JSNode| {
            let core_node: Rc<RefCell<dyn Source>> = match node {
                JSNode::MultiplyNodeJS(multiply_node_js) => rc_refcell!(MultiplyNode::new(
                    multiply_node_js.metadata.id,
                    multiply_node_js.multiplicand_source_id,
                    multiply_node_js.multiplier_source_id
                )),
                JSNode::SumNodeJS(sum_node_js) => rc_refcell!(SumNode::new(
                    sum_node_js.metadata.id,
                    sum_node_js.augend_source_id,
                    sum_node_js.addend_source_id
                )),
                JSNode::SineOscillatorNodeJS(sine_oscillator_node_js) => {
                    rc_refcell!(SineOscillatorNode::new(
                        sine_oscillator_node_js.metadata.id,
                        sine_oscillator_node_js.sine_frequency_source_id
                    ))
                }
                JSNode::SawOscillatorNodeJS(saw_oscillator_node_js) => {
                    rc_refcell!(SawOscillatorNode::new(
                        saw_oscillator_node_js.metadata.id,
                        saw_oscillator_node_js.saw_frequency_source_id
                    ))
                }
                JSNode::SquareOscillatorNodeJS(square_oscillator_node_js) => {
                    rc_refcell!(SquareOscillatorNode::new(
                        square_oscillator_node_js.metadata.id,
                        square_oscillator_node_js.square_frequency_source_id
                    ))
                }
                JSNode::FloatSourceJS(float_source_js) => rc_refcell!(FloatSource::new(
                    float_source_js.metadata.id,
                    float_source_js.value
                )),
                JSNode::SplineFloatNode(spline_float_node) => {
                    rc_refcell!(SplineFloatNode::new(
                        spline_float_node.metadata.id,
                        spline_float_node.frequency_source_id,
                        spline_float_node
                            .points
                            .iter()
                            .map(|point| (point.x, point.y))
                            .collect()
                    ))
                }
            };
            core_node
        })
        .collect();

    SampleGenerator::new(nodes, AudioContext::new(graph.audio_context.sample_rate)).unwrap() // TODO: Hook into JS exception throwing to avoid unwrap() call
}
