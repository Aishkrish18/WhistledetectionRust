use fon::{mono::Mono32, Audio};
use pasts::{exec, wait};
use spectrum_analyzer::{samples_fft_to_spectrum, windows::hann_window, FrequencyLimit};
use wavy::{Microphone, MicrophoneStream};

enum Event<'a> {
    Record(MicrophoneStream<'a, Mono32>),
    Analyze(),
}

/// Shared state between tasks on the thread.
struct State {
    buffer: Audio<Mono32>,
    sample_length: usize,
    frequency_limit: FrequencyLimit,
}

impl State {
    fn event(&mut self, event: Event<'_>) {
        match event {
            Event::Record(microphone) => {
                //println!("Recording");
                self.buffer.extend(microphone);
            }
            Event::Analyze() => {
                if self.buffer.len() >= self.sample_length {
                    //self.buffer.drain();

                    let samples = self.buffer.as_f32_slice();
                    let sample = &samples[0..self.sample_length];

                    let hann_window = hann_window(&sample);
                    let frequency_spectrum = samples_fft_to_spectrum(
                        &hann_window,
                        48000,
                        self.frequency_limit,
                        Some(&|val, info| val - info.min),
                    )
                    .unwrap();

                    for (fr, fr_val) in frequency_spectrum.data().iter() {
                        println!("{}Hz => {}", fr, fr_val)
                    }
                    std::process::exit(0);

                    
                }

                //println!("{:?}", self.buffer.sample_rate());
                println!("{:?}", self.buffer.len());
            }
        }
    }
}

fn main() {
    let mut state = State {
        buffer: Audio::with_silence(48_000, 0),
        sample_length: 4096,
        frequency_limit: FrequencyLimit::Range(200.0, 10000.0),
    };
    let mut microphone = Microphone::default();

    exec!(state.event(wait! {
        Event::Record(microphone.record().await),
        Event::Analyze(),
    }))
}
