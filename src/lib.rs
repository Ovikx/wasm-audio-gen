use std::{cell::RefCell, panic, rc::Rc};

use audio_gen::{
    context::audio_context::AudioContext, generator::SampleGenerator, node::{
        float::FloatSource, multiply::MultiplyNode, saw_oscillator::SawOscillatorNode, sine_oscillator::SineOscillatorNode, spline_float::SplineFloatNode, sum::SumNode
    }, source::Source
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{graph::SourceGraph, js_node::JSNode,};

extern crate console_error_panic_hook;
extern crate web_sys;

pub mod graph;
pub mod js_node;
pub mod metadata;
pub mod nodes;

macro_rules! rc_refcell {
    ($expr:expr) => {
        Rc::new(RefCell::new($expr))
    };
}

#[wasm_bindgen]
pub struct WasmSampleGenerator {
    generator: SampleGenerator,
}

#[wasm_bindgen]
impl WasmSampleGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let mut graph = audio_gen::graph::Graph::new();
        let float_source_id = graph.insert_float_node(440.);
        graph.insert_sine_oscillator_node(float_source_id);
        WasmSampleGenerator {
            generator: SampleGenerator::new(graph.nodes(), AudioContext::new(44100.)).unwrap(),
        }
    }

    pub fn load_graph(&mut self, graph: JsValue) {
        let graph: SourceGraph = serde_wasm_bindgen::from_value(graph).unwrap();
        let nodes: Vec<Rc<RefCell<dyn Source>>> =  graph.nodes.iter().map(|node: &JSNode| {
            match node {
                JSNode::MultiplyNodeJS(multiply_node_js) => rc_refcell!(MultiplyNode::new(multiply_node_js.metadata.id, multiply_node_js.multiplicand_source_id, multiply_node_js.multiplier_source_id)) as Rc<RefCell<dyn Source>>,
                JSNode::SumNodeJS(sum_node_js) => rc_refcell!(SumNode::new(sum_node_js.metadata.id, sum_node_js.augend_source_id, sum_node_js.addend_source_id)) as Rc<RefCell<dyn Source>>,
                JSNode::SineOscillatorNodeJS(sine_oscillator_node_js) => rc_refcell!(SineOscillatorNode::new(sine_oscillator_node_js.metadata.id, sine_oscillator_node_js.sine_frequency_source_id)),
                JSNode::SawOscillatorNodeJS(saw_oscillator_node_js) => rc_refcell!(SawOscillatorNode::new(saw_oscillator_node_js.metadata.id, saw_oscillator_node_js.saw_frequency_source_id)),
                JSNode::FloatSourceJS(float_source_js) => rc_refcell!(FloatSource::new(float_source_js.metadata.id, float_source_js.value)),
                JSNode::SplineFloatNode(spline_float_node) => rc_refcell!(SplineFloatNode::new(spline_float_node.metadata.id, spline_float_node.frequency_source_id, spline_float_node.points.iter().map(|point| (point.x, point.y)).collect())),
            }
        }).collect();

        self.generator = SampleGenerator::new(
            nodes,
            AudioContext::new(44100.),
        ).unwrap();
    }

    pub fn generate_samples(&mut self, num_samples: u32) -> Vec<f32> {
        self.generator.batch_poll(num_samples)
    }
}
