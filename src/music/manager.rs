use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn note_freq(note: &str) -> f32 {
    match note {
        "A0" => 27.50,
        "A#0" => 29.14,
        "B♭0" => 29.14,
        "B0" => 30.87,
        "C1" => 32.70,
        "C#1" => 34.65,
        "D1" => 36.71,
        "D#1" => 38.89,
        "E♭1" => 38.89,
        "E1" => 41.20,
        "F1" => 43.65,
        "F#1" => 46.25,
        "G1" => 49.00,
        "G#1" => 51.91,
        "A1" => 55.00,
        "A#1" => 58.27,
        "B♭1" => 58.27,
        "B1" => 61.74,
        "C2" => 65.41,
        "C#2" => 69.30,
        "D2" => 73.42,
        "D#2" => 77.78,
        "E♭2" => 77.78,
        "E2" => 82.41,
        "F2" => 87.31,
        "F#2" => 92.50,
        "G2" => 98.00,
        "G#2" => 103.83,
        "A2" => 110.00,
        "A#2" => 116.54,
        "B♭2" => 116.54,
        "B2" => 123.47,
        "C3" => 130.81,
        "C#3" => 138.59,
        "D3" => 146.83,
        "D#3" => 155.56,
        "E♭3" => 155.56,
        "E3" => 164.81,
        "F3" => 174.61,
        "F#3" => 185.00,
        "G3" => 196.00,
        "G#3" => 207.65,
        "A3" => 220.00,
        "A#3" => 233.08,
        "B♭3" => 233.08,
        "B3" => 246.94,
        "C4" => 261.63,
        "C#4" => 277.18,
        "D4" => 293.66,
        "D#4" => 311.13,
        "E♭4" => 311.13,
        "E4" => 329.63,
        "F4" => 349.23,
        "F#4" => 369.99,
        "G4" => 392.00,
        "G#4" => 415.30,
        "A4" => 440.00,
        "A#4" => 466.16,
        "B♭4" => 466.16,
        "B4" => 493.88,
        "C5" => 523.25,
        "C#5" => 554.37,
        "D5" => 587.33,
        "D#5" => 622.25,
        "E♭5" => 622.25,
        "E5" => 659.26,
        "F5" => 698.46,
        "F#5" => 739.99,
        "G5" => 783.99,
        "G#5" => 830.61,
        "A5" => 880.00,
        "A#5" => 932.33,
        "B♭5" => 932.33,
        "B5" => 987.77,
        "C6" => 1046.50,
        "C#6" => 1108.73,
        "D6" => 1174.66,
        "D#6" => 1244.51,
        "E♭6" => 1244.51,
        "E6" => 1318.51,
        "F6" => 1396.91,
        "F#6" => 1479.98,
        "G6" => 1567.98,
        "G#6" => 1661.22,
        "A6" => 1760.00,
        "A#6" => 1864.66,
        "B♭6" => 1864.66,
        "B6" => 1975.53,
        "C7" => 2093.00,
        "C#7" => 2217.46,
        "D7" => 2349.32,
        "D#7" => 2489.02,
        "E♭7" => 2489.02,
        "E7" => 2637.02,
        "F7" => 2793.83,
        "F#7" => 2959.96,
        "G7" => 3135.96,
        "G#7" => 3322.44,
        "A7" => 3520.00,
        "A#7" => 3729.31,
        "B♭7" => 3729.31,
        "B7" => 3951.07,
        "C8" => 4186.01,
        _ => 0.0,
    }
}

const MELODY: [(&str, u64); 124] = [
    ("B♭4", 500), // "Look"
    ("G4", 500),  // "at"
    // bar
    ("D4", 3000), // "me"
    ("B♭3", 500), // "I'm"
    ("C4", 500),  // "as"
    // bar
    ("C#4", 500), // "help-"
    ("C5", 500),  // "less-"
    ("C5", 500),  // "as"
    ("A#4", 500), // "a"
    ("C5", 500),  // "kitt-"
    ("A#4", 500), // "-en"
    ("G4", 500),  // up
    ("E♭4", 500), // a
    // bar
    ("C4", 2500), // tree
    ("", 250),    // (quarter-note rest)
    ("G#3", 250), // and
    ("G#3", 250), // I
    ("G#3", 250), // feel
    ("C4", 250),  // like
    ("E♭4", 250), // I'm
    // bar
    ("B♭4", 500),  // cling-
    ("B♭4", 500),  // ing
    ("B♭4", 500),  // to
    ("G#4", 500),  // a
    ("B♭4", 1500), // cloud
    ("G#4", 500),  // I
    // bar
    ("G4", 1000),  // can't
    ("G#4", 500),  // un-
    ("B♭4", 500),  // -der-
    ("E♭4", 1000), // -stand
    ("F4", 500),   // I
    ("G4", 500),   // get
    // bar
    ("G#4", 500), // mist-
    ("C4", 1000), // -y
    ("E♭4", 500), // just
    ("D4", 500),  // hold-
    ("G4", 500),  // -ing
    ("C5", 500),  // your
    // bar
    ("A4", 4000), // hand
    // bar
    ("A4", 3000), // hand
    ("B♭4", 500), // "Walk"
    ("G4", 500),  // "my"
    // bar
    ("D4", 3000), // "way"
    ("B♭3", 500), // "and"
    ("C4", 500),  // "a"
    // bar
    ("C#4", 500), // "thou-"
    ("C5", 500),  // "-sand"
    ("C5", 500),  // "vi-"
    ("A#4", 500), // "-o-"
    ("C5", 500),  // "lins"
    ("A#4", 500), // "be"
    ("G4", 500),  // gin
    ("E♭4", 500), // to
    // bar
    ("C4", 3000), // play
    ("G#3", 250), // or
    ("G#3", 250), // it
    ("G#3", 250), // might
    ("C4", 250),  // be
    ("E♭4", 250), // the
    // bar
    ("B♭4", 500),  // sound
    ("B♭4", 500),  // of
    ("B♭4", 500),  // your
    ("G#4", 500),  // hell-
    ("B♭4", 1000), // o
    ("G#4", 1000), // that
    ("G4", 1000),  // mu-
    ("G#4", 500),  // -sic
    ("B♭4", 500),  // I
    ("E♭4", 1000), // hear
    ("F4", 500),   // I
    ("G4", 500),   // get
    ("G#4", 500),  // mis
    ("C4", 1000),  // -ty
    ("E♭4", 500),  // when-
    ("D4", 500),   // -ev-
    ("E♭4", 500),  // -er
    ("F4", 1000),  // you're
    ("E♭4", 3000), // near
    ("", 4500),    // --pause--
    ("E♭4", 500),  // Don't
    ("F4", 500),   // you
    ("G4", 500),   // see
    ("B♭4", 500),  // that
    ("C5", 500),   // you're
    ("C#5", 1500), // lead-
    ("C#5", 500),  // -ing
    ("C#5", 500),  // me
    ("C#5", 2000), // on?
    ("C#5", 500),  // and
    ("E♭5", 500),  // it's
    ("E5", 1000),  // just
    ("E♭5", 500),  // what
    ("C#5", 500),  // I
    ("C5", 1000),  // want
    ("C5", 500),   // you
    ("E♭4", 500),  // to
    ("C5", 2000),  // do?
    ("", 3000),    // --pause--
    ("E♭4", 500),  // Don't
    ("F4", 500),   // you
    ("G4", 500),   // no-
    ("B♭4", 500),  // -tice
    ("C5", 500),   // how
    ("D5", 500),   // hope-
    ("D5", 500),   // -less-
    ("D5", 500),   // -ly
    ("D5", 500),   // I'm
    ("D5", 2000),  // lost
    ("D5", 500),   // That's
    ("D5", 500),   // Why
    ("C#5", 500),  // I'm
    ("D5", 500),   // foll-
    ("F5", 500),   // -oll-
    ("D5", 500),   // -ow-
    ("C5", 500),   // ing
    ("B♭4", 1000), // you
    ("B♭4", 1000), // --riff--
    ("G#4", 1000), // --riff--
    ("G4", 1000),  // --riff--
    ("G#4", 1000), // --riff--
    ("G4", 1000),  // --riff--
    ("F4", 1000),  // --riff--
    ("B♭4", 1000), // --riff--
    ("", 3000),    // --pause--
];

const BASSLINE: [(&str, u64); 33] = [
    ("", 1000), // 2-beat lead for melody...
    ("E♭2", 1000),
    ("G2", 1000),
    ("A♭2", 1000),
    ("B♭2", 1000), // E♭maj7
    ("C2", 1000),
    ("E♭2", 1000),
    ("G2", 1000),
    ("A2", 1000), // Cmin7
    ("F2", 1000),
    ("A2", 1000),
    ("C3", 1000),
    ("D3", 1000), // Fmin7
    ("B♭2", 1000),
    ("D3", 1000),
    ("F3", 1000),
    ("G3", 1000), // B♭7
    ("E♭2", 1000),
    ("G2", 1000),
    ("A♭2", 1000),
    ("B♭2", 1000), // E♭maj7
    ("C2", 1000),
    ("E♭2", 1000),
    ("G2", 1000),
    ("A2", 1000), // Cmin7
    ("F2", 1000),
    ("A2", 1000),
    ("C3", 1000),
    ("D3", 1000), // Fmin7
    ("B♭2", 1000),
    ("D3", 1000),
    ("F3", 1000),
    ("G3", 1000), // B♭7
];

pub fn play_sound() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config().unwrap();
    let sample_rate = config.sample_rate().0 as f32;

    let shared_state = Arc::new(Mutex::new((0, 0, 0, 0))); // (melody index, bass index, melody samples, bass samples)

    let stream = {
        let state = Arc::clone(&shared_state);

        device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut state = state.lock().unwrap();
                    let (mut melody_index, mut bass_index, mut melody_counter, mut bass_counter) =
                        *state;

                    let melody_duration_samples =
                        (MELODY[melody_index].1 as f32 / 1000.0) * sample_rate;
                    let bass_duration_samples =
                        (BASSLINE[bass_index].1 as f32 / 1000.0) * sample_rate;

                    // Change melody note if necessary
                    if melody_counter as f32 >= melody_duration_samples {
                        melody_index = (melody_index + 1) % MELODY.len();
                        melody_counter = 0;
                    }
                    // Change bass note if necessary
                    if bass_counter as f32 >= bass_duration_samples {
                        bass_index = (bass_index + 1) % BASSLINE.len();
                        bass_counter = 0;
                    }

                    let melody_freq = note_freq(MELODY[melody_index].0);
                    let bass_freq = note_freq(BASSLINE[bass_index].0);

                    // Define volumes (adjust these as needed)
                    let melody_volume = 0.2;
                    let bass_volume = 0.1;

                    for sample in data.iter_mut() {
                        // let melody_wave =
                        //     generate_square_wave(melody_counter, melody_freq, sample_rate);
                        let melody_wave =
                            generate_pulse_wave(melody_counter, melody_freq, sample_rate, 0.25);

                        let bass_wave =
                            generate_triangle_wave(bass_counter, bass_freq, sample_rate);

                        // **Now YOU control the mix**
                        *sample = (melody_wave * melody_volume) + (bass_wave * bass_volume);

                        melody_counter += 1;
                        bass_counter += 1;
                    }

                    *state = (melody_index, bass_index, melody_counter, bass_counter);
                },
                |err| eprintln!("Error: {}", err),
                None,
            )
            .unwrap()
    };

    stream.play().unwrap();
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}

// 🎵 Generate a square wave (used for melody)
fn generate_square_wave(sample_counter: u64, freq: f32, sample_rate: f32) -> f32 {
    if (sample_counter as f32 * freq * 2.0 * std::f32::consts::PI / sample_rate).sin() > 0.0 {
        1.0
    } else {
        -1.0
    }
}

// 🎵 Generate a triangle wave (used for bass)
fn generate_triangle_wave(sample_counter: u64, freq: f32, sample_rate: f32) -> f32 {
    let t = (sample_counter as f32 * freq / sample_rate) % 1.0;
    (2.0 * (t - 0.5)).abs() * 2.0 - 1.0
}

fn generate_pulse_wave(sample_counter: u64, freq: f32, sample_rate: f32, duty_cycle: f32) -> f32 {
    let period = sample_rate / freq;
    if (sample_counter as f32 % period) < (duty_cycle * period) {
        1.0
    } else {
        -1.0
    }
}
