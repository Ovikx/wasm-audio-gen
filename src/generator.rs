use audio_gen::{context::AudioContext, generator::SampleGenerator};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::build_generator::build_generator;

#[wasm_bindgen]
pub struct WasmSampleGenerator {
    generator: SampleGenerator,
}

#[wasm_bindgen]
impl WasmSampleGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut graph = audio_gen::graph::Graph::new();
        let float_source_id = graph.insert_float_node(440.);
        graph.insert_sine_oscillator_node(float_source_id);
        WasmSampleGenerator {
            generator: SampleGenerator::new(graph.nodes(), AudioContext::new(44100.)).unwrap(),
        }
    }

    pub fn load_graph(&mut self, graph: JsValue) {
        self.generator = build_generator(graph);
    }

    pub fn generate_samples(&mut self, num_samples: u32) -> Vec<f32> {
        self.generator.batch_poll(num_samples)
    }
}
