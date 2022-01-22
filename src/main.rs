// This example records audio for 5 seconds and writes to a raw PCM file.

use fon::{mono::Mono32, Audio, Frame};
use pasts::{exec, wait};
use wavy::{Microphone, MicrophoneStream};

/// An event handled by the event loop.
enum Event<'a> {
    /// Microphone has recorded some audio.
    Record(MicrophoneStream<'a, Mono32>),
}

/// Shared state between tasks on the thread.
struct State {
    /// Temporary buffer for holding real-time audio samples.
    buffer: Audio<Mono32>,
    buffer_length: usize
}

impl State {
    /// Event loop.  Return false to stop program.
    fn event(&mut self, event: Event<'_>) {
        match event {
            Event::Record(microphone) => {
                println!("Recording");
                self.buffer.extend(microphone);
                if self.buffer.len() >= self.buffer_length {
                    println!("Finished");
                    std::process::exit(0);
                }
            }
        }
    }
}



/// Program start.
fn main() {
    let mut state = State {
        buffer: Audio::with_silence(48_000, 0),
        buffer_length: (48000.0 * 1.5) as usize
    };
    let mut microphone = Microphone::default();
    exec!(state.event(wait!{Event::Record(microphone.record().await),
    }))
}