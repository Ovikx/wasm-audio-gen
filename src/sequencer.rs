use std::cell::RefCell;

use audio_gen::{
    context::AudioContext,
    generator::SampleGenerator,
    sequencer::{GeneratorInterval, Sequencer as CoreSequencer},
};
use js_sys::Float32Array;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{build_nodes::build_nodes, deserializable::sequence::GeneratorSequence};

#[wasm_bindgen]
pub struct Sequencer {
    sequencer: CoreSequencer,
}

#[wasm_bindgen]
impl Sequencer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Sequencer {
            sequencer: CoreSequencer::new(vec![]),
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
                            build_nodes(interval.nodes.clone()),
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
