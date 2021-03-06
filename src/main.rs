use fon::chan::Ch32;
use fon::mono::Mono;
use fon::{mono::Mono32, Audio};
use spectrum_analyzer::{samples_fft_to_spectrum, windows::hann_window, FrequencyLimit};
use wavy::{Microphone, MicrophoneStream};

use std::fs::File;
use std::io::prelude::*;

struct State {
    buffer: Audio<Mono32>,
    sample_length: usize,
    frequency_limit: FrequencyLimit,
    is_whistle_detected: bool,
    threshold_base: f32,
    threshold_overtone_1: f32,
    threshold_overtone_2: f32,
}

fn record(state: &mut State, stream: MicrophoneStream<Mono<Ch32>>) {
    state.buffer.extend(stream);
}

fn analyze(state: &mut State) {
    if state.buffer.len() >= state.sample_length {
        // buffer is full enough

        let buffer_slice = state.buffer.as_f32_slice();
        let samples = &buffer_slice[(buffer_slice.len() - state.sample_length)..buffer_slice.len()];

        let hann_window = hann_window(&samples);
        let frequency_spectrum = samples_fft_to_spectrum(
            &hann_window,
            48000,
            state.frequency_limit,
            Some(&|val, info| val - info.min),
        )
        .unwrap();

        let mut base = false;
        let mut overtone_1 = false;
        let mut overtone_2 = false;

        //let mut file = File::create("exported_spectrum.csv").unwrap();

        for (fr, fr_val) in frequency_spectrum.data().iter() {
            if fr.val() >= 2000.0 && fr.val() <= 4000.0 {
                if fr_val.val() > state.threshold_base {
                    //println!("Base threshold is reached {:?}", fr);
                    base = true;
                }
            } else if fr.val() > 4000.0 && fr.val() <= 5000.0 {
                if fr_val.val() > state.threshold_overtone_1 {
                    //println!("First Threshold Overtone is reached {:?}", fr);
                    overtone_1 = true;
                }
            } else if fr.val() > 6000.0 && fr.val() <= 7200.0 {
                if fr_val.val() > state.threshold_overtone_2 {
                    //println!("Second Threshold Overtone is reached {:?}", fr);
                    overtone_2 = true;
                }
            }

            //let data = format!("{} {} ", fr.val(), fr_val.val());
            //write!(file, "{}", data);

            //println!("{}Hz => {}", fr, fr_val)
        }

        if base && overtone_1 && overtone_2 {
            state.is_whistle_detected = true;
            //std::process::exit(0);
        }

        state.buffer = Audio::with_silence(48_000, 0); // clear buffer
    }
}

#[tokio::main]
async fn main() {
    let mut state = State {
        buffer: Audio::with_silence(48_000, 0),
        sample_length: 4096, // powers of 2, maximal 4096
        frequency_limit: FrequencyLimit::Range(1000.0, 7500.0),
        is_whistle_detected: false,
        threshold_base: 50.0, // yet to be found
        threshold_overtone_1: 0.0,
        threshold_overtone_2: 0.0,
    };

    let mut microphone = Microphone::default();

    while !state.is_whistle_detected {
        record(&mut state, microphone.record::<Mono32>().await);
        analyze(&mut state);
        println!("Whistle detected: {:?}", state.is_whistle_detected);
    }
}
