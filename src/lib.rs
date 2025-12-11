use std::{cell::RefCell, collections::HashMap, panic, rc::Rc};

use audio_gen::{
    context::audio_context::AudioContext,
    generator::SampleGenerator,
    node::{
        float::Float32Source, multiply::MultiplyNode, saw_oscillator::SawOscillatorNode, sine_oscillator::SineOscillatorNode, spline_float::SplineFloatNode, sum::SumNode
    },
    source::{CachedFloatSource, SharedCachedFloatSource}
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{graph::SourceGraph, js_node::JSNode, metadata::WithMetadata};

extern crate console_error_panic_hook;
extern crate web_sys;

pub mod graph;
pub mod js_node;
pub mod metadata;
pub mod nodes;

#[wasm_bindgen]
pub struct WasmSampleGenerator {
    generator: SampleGenerator,
}

#[wasm_bindgen]
impl WasmSampleGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let float_source = Float32Source::new(880.0);
        let oscillator = SineOscillatorNode::new(rc_refcell_source(CachedFloatSource::new(Box::new(float_source))));
        WasmSampleGenerator {
            generator: SampleGenerator::new(rc_refcell_source(CachedFloatSource::new(Box::new(oscillator))), AudioContext::new(44100.)),
        }
    }

    pub fn load_graph(&mut self, graph: JsValue) {
        let graph: SourceGraph = serde_wasm_bindgen::from_value(graph).unwrap();
        let mut id_to_js_node: HashMap<i32, JSNode> = HashMap::new();
        let mut id_to_node: HashMap<i32, SharedCachedFloatSource> = HashMap::new();
        for node in graph.nodes {
            id_to_js_node.insert(node.metadata().id, node);
        }

        let mut js_nodes_to_materialize: Vec<i32> = vec![graph.root_id];
        while js_nodes_to_materialize.len() > 0 {
            let popped_js_node = id_to_js_node
                .get(&js_nodes_to_materialize.pop().unwrap())
                .unwrap();
            match popped_js_node {
                JSNode::MultiplyNodeJS(node) => {
                    let multiplicand_node_exists =
                        id_to_node.contains_key(&node.multiplicand_source_id);
                    let multiplier_node_exists =
                        id_to_node.contains_key(&node.multiplier_source_id);
                    if multiplicand_node_exists && multiplier_node_exists {
                        let multiplicand_source =
                            Rc::clone(id_to_node.get(&node.multiplicand_source_id).unwrap());
                        let multiplier_source =
                            Rc::clone(id_to_node.get(&node.multiplier_source_id).unwrap());
                        let multiply_node =
                            MultiplyNode::new(multiplicand_source, multiplier_source);
                        id_to_node.insert(node.metadata.id, rc_refcell_source(CachedFloatSource::new(Box::new(multiply_node))));
                    } else {
                        js_nodes_to_materialize.push(node.metadata.id);
                        if !multiplicand_node_exists {
                            js_nodes_to_materialize.push(node.multiplicand_source_id);
                        }
                        if !multiplier_node_exists {
                            js_nodes_to_materialize.push(node.multiplier_source_id);
                        }
                    }
                }
                JSNode::SumNodeJS(node) => {
                    let augend_node_exists = id_to_node.contains_key(&node.augend_source_id);
                    let addend_node_exists = id_to_node.contains_key(&node.addend_source_id);
                    if augend_node_exists && addend_node_exists {
                        let augend_source =
                            Rc::clone(id_to_node.get(&node.augend_source_id).unwrap());
                        let addend_source =
                            Rc::clone(id_to_node.get(&node.addend_source_id).unwrap());
                        let sum_node = SumNode::new(augend_source, addend_source);
                        id_to_node.insert(node.metadata.id, rc_refcell_source(CachedFloatSource::new(Box::new(sum_node))));
                    } else {
                        js_nodes_to_materialize.push(node.metadata.id);
                        if !augend_node_exists {
                            js_nodes_to_materialize.push(node.augend_source_id);
                        }
                        if !addend_node_exists {
                            js_nodes_to_materialize.push(node.addend_source_id);
                        }
                    }
                }
                JSNode::SineOscillatorNodeJS(node) => {
                    if id_to_node.contains_key(&node.sine_frequency_source_id) {
                        let frequency_source =
                            Rc::clone(id_to_node.get(&node.sine_frequency_source_id).unwrap());
                        let oscillator_node = SineOscillatorNode::new(frequency_source);
                        id_to_node.insert(node.metadata.id, rc_refcell_source(CachedFloatSource::new(Box::new(oscillator_node))));
                    } else {
                        js_nodes_to_materialize
                            .extend([node.metadata.id, node.sine_frequency_source_id]);
                    }
                }
                JSNode::SawOscillatorNodeJS(node) => {
                    if id_to_node.contains_key(&node.saw_frequency_source_id) {
                        let frequency_source =
                            Rc::clone(id_to_node.get(&node.saw_frequency_source_id).unwrap());
                        let oscillator_node = SawOscillatorNode::new(frequency_source);
                        id_to_node.insert(node.metadata.id, rc_refcell_source(CachedFloatSource::new(Box::new(oscillator_node))));
                    } else {
                        js_nodes_to_materialize
                            .extend([node.metadata.id, node.saw_frequency_source_id]);
                    }
                }
                JSNode::Float32SourceJS(node) => {
                    id_to_node.insert(
                        node.metadata.id,
                        rc_refcell_source(CachedFloatSource::new(Box::new(Float32Source::new(node.value)))),
                    );
                }
                JSNode::SplineFloatNode(node) => {
                    if id_to_node.contains_key(&node.frequency_source_id) {
                        let frequency_source =
                            Rc::clone(id_to_node.get(&node.frequency_source_id).unwrap());
                        let spline_node = SplineFloatNode::new(frequency_source, node.points.iter().map(|point| (point.x, point.y)).collect());
                        id_to_node.insert(node.metadata.id, rc_refcell_source(CachedFloatSource::new(Box::new(spline_node))));
                    } else {
                        js_nodes_to_materialize
                            .extend([node.metadata.id, node.frequency_source_id]);
                    }
                }
            }
        }

        self.generator = SampleGenerator::new(
            Rc::clone(id_to_node.get(&graph.root_id).unwrap()),
            AudioContext::new(44100.),
        );
    }

    pub fn generate_samples(&mut self, num_samples: u32) -> Vec<f32> {
        self.generator.generate_samples(num_samples)
    }
}

fn rc_refcell_source(value: CachedFloatSource) -> SharedCachedFloatSource {
    Rc::new(RefCell::new(value))
}
