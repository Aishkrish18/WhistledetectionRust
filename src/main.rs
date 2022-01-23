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
    is_whistle_detected: bool,
    threshold_base: f32,
    threshold_overtone_1: f32,
    threshold_overtone_2: f32,
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
                    // let buffer_stream = self.buffer.drain();
                    // let test = buffer_stream.take(4);

                    let buffer_slice = self.buffer.as_f32_slice();
                    let samples = &buffer_slice
                        [(buffer_slice.len() - self.sample_length)..buffer_slice.len()];

                    let hann_window = hann_window(&samples);
                    let frequency_spectrum = samples_fft_to_spectrum(
                        &hann_window,
                        48000,
                        self.frequency_limit,
                        Some(&|val, info| val - info.min),
                    )
                    .unwrap();

                    let mut base = false;
                    let mut overtone_1 = false;
                    let mut overtone_2 = false;
                    for (fr, fr_val) in frequency_spectrum.data().iter() {
                        if fr.val() >= 2000.0 && fr.val() <= 4000.0 {
                            if fr_val.val() > self.threshold_base {
                                println!("Base threshold is reached {:?}", fr);
                                base = true;
                            }
                        } else if fr.val() > 4000.0 && fr.val() <= 5000.0 {
                            if fr_val.val() > self.threshold_overtone_1 {
                                println!("First Threshold Overtone is reached {:?}", fr);
                                overtone_1 = true;
                            }
                        } else if fr.val() > 6000.0 && fr.val() <= 7200.0 {
                            if fr_val.val() > self.threshold_overtone_2 {
                                println!("Second Threshold Overtone is reached {:?}", fr);
                                overtone_2 = true;
                            }
                        }

                        //println!("{}Hz => {}", fr, fr_val)
                    }

                    self.buffer = Audio::with_silence(48_000, 0);

                    //let weighted_range_sum = 2.3;
                    //if weighted_range_sum > self.threshold{
                    //    self.is_whistle_detected = true;

                    //    std::process::exit(0);
                    //}

                    println!("{:?}", self.is_whistle_detected);
                }

                //println!("{:?}", self.buffer.sample_rate());
                //println!("{:?}", self.buffer.len());
            }
        }
    }
}

fn main() {
    let mut state = State {
        buffer: Audio::with_silence(48_000, 0),
        sample_length: 4096,
        frequency_limit: FrequencyLimit::Range(1000.0, 7500.0),
        is_whistle_detected: false,
        threshold_base: 5.0,
        threshold_overtone_1: 2.0,
        threshold_overtone_2: 1.0,
    };
    let mut microphone = Microphone::default();

    exec!(state.event(wait! {
        Event::Record(microphone.record().await),
        Event::Analyze(),
    }));

    //println!("Whistle detected: {:?}", state.event(wait!{Event::Analyze().is_whistle_detected);
}
