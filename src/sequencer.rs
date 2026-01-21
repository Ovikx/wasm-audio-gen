use std::cell::RefCell;

use audio_gen::{
    context::AudioContext,
    generator::SampleGenerator,
    input_buffer::{ExternalInputBuffer, SharedExternalInputBuffer},
    sequencer::{GeneratorInterval, Sequencer as CoreSequencer},
};
use js_sys::Float32Array;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{build_nodes::build_nodes, deserializable::sequence::GeneratorSequence};

#[wasm_bindgen]
pub struct Sequencer {
    sequencer: CoreSequencer,
    input_buffer: SharedExternalInputBuffer,
}

#[wasm_bindgen]
impl Sequencer {
    #[wasm_bindgen(constructor)]
    pub fn new(input_buffer_size: usize) -> Self {
        Sequencer {
            sequencer: CoreSequencer::new(vec![]),
            input_buffer: ExternalInputBuffer::new_shared(input_buffer_size),
        }
    }

    #[wasm_bindgen]
    pub fn load_sequence(&mut self, js_sequence: JsValue) {
        let sequence: GeneratorSequence = serde_wasm_bindgen::from_value(js_sequence).unwrap();
        let generator_intervals: Vec<GeneratorInterval> = sequence
            .generator_intervals
            .iter()
            .map(|interval| {
                GeneratorInterval::new(
                    RefCell::new(
                        SampleGenerator::new(
                            build_nodes(interval.nodes.clone(), self.input_buffer.clone()),
                            AudioContext::new(sequence.audio_context.sample_rate),
                        )
                        .unwrap(),
                    ),
                    interval.start_sample,
                    interval.end_sample,
                )
            })
            .collect();

        self.sequencer = CoreSequencer::new(generator_intervals)
    }

    #[wasm_bindgen]
    pub fn generate_samples(&mut self, num_samples: u32) -> SequencerPollResult {
        let mut samples = vec![];
        let mut done = false;
        for _ in 0..num_samples {
            match self.sequencer.poll() {
                Some(sample) => samples.push(sample),
                None => {
                    samples.push(0.);
                    done = true;
                }
            }
        }

        SequencerPollResult { samples, done }
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

#[wasm_bindgen]
pub struct SequencerPollResult {
    samples: Vec<f32>,
    done: bool,
}

#[wasm_bindgen]
impl SequencerPollResult {
    #[wasm_bindgen(getter)]
    pub fn samples(&self) -> Float32Array {
        Float32Array::from(self.samples.as_slice())
    }

    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.done
    }
}
