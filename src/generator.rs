use audio_gen::{
    context::AudioContext,
    generator::SampleGenerator as CoreSampleGenerator,
    input_buffer::{ExternalInputBuffer, SharedExternalInputBuffer},
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{build_nodes::build_nodes, deserializable::source_graph::ConfiguredSourceGraph};

#[wasm_bindgen]
pub struct SampleGenerator {
    generator: CoreSampleGenerator,
    input_buffer: SharedExternalInputBuffer,
}

#[wasm_bindgen]
impl SampleGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(input_buffer_size: usize) -> Self {
        let mut graph = audio_gen::graph::Graph::new();
        let float_source_id = graph.insert_float_node(440.);
        graph.insert_sine_oscillator_node(float_source_id);
        SampleGenerator {
            generator: CoreSampleGenerator::new(graph.nodes(), AudioContext::new(44100.)).unwrap(),
            input_buffer: ExternalInputBuffer::new_shared(input_buffer_size),
        }
    }

    pub fn load_graph(&mut self, js_graph: JsValue) {
        let graph: ConfiguredSourceGraph = serde_wasm_bindgen::from_value(js_graph).unwrap();
        self.generator = CoreSampleGenerator::new(
            build_nodes(graph.nodes, self.input_buffer.clone()),
            AudioContext::new(graph.audio_context.sample_rate),
        )
        .unwrap();
    }

    pub fn generate_samples(&mut self, num_samples: u32) -> Vec<f32> {
        self.generator.batch_poll(num_samples)
    }

    pub fn update_bool(&mut self, index: usize, new_value: bool) {
        self.input_buffer
            .borrow_mut()
            .update_bool(index, new_value)
            .unwrap()
    }

    pub fn update_f32(&mut self, index: usize, new_value: f32) {
        self.input_buffer
            .borrow_mut()
            .update_f32(index, new_value)
            .unwrap()
    }

    pub fn update_u32(&mut self, index: usize, new_value: u32) {
        self.input_buffer
            .borrow_mut()
            .update_u32(index, new_value)
            .unwrap()
    }
}
