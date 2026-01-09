use audio_gen::{context::AudioContext, generator::SampleGenerator as CoreSampleGenerator};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{build_nodes::build_nodes, deserializable::source_graph::ConfiguredSourceGraph};

#[wasm_bindgen]
pub struct SampleGenerator {
    generator: CoreSampleGenerator,
}

#[wasm_bindgen]
impl SampleGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut graph = audio_gen::graph::Graph::new();
        let float_source_id = graph.insert_float_node(440.);
        graph.insert_sine_oscillator_node(float_source_id);
        SampleGenerator {
            generator: CoreSampleGenerator::new(graph.nodes(), AudioContext::new(44100.)).unwrap(),
        }
    }

    pub fn load_graph(&mut self, js_graph: JsValue) {
        let graph: ConfiguredSourceGraph = serde_wasm_bindgen::from_value(js_graph).unwrap();
        self.generator = CoreSampleGenerator::new(
            build_nodes(graph.nodes),
            AudioContext::new(graph.audio_context.sample_rate),
        )
        .unwrap();
    }

    pub fn generate_samples(&mut self, num_samples: u32) -> Vec<f32> {
        self.generator.batch_poll(num_samples)
    }
}
