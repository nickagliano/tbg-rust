use crate::music::misty;
use crate::music::synth::{generate_pulse_wave, generate_triangle_wave, note_freq};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

pub struct MusicPlayer {
    music_playing: Arc<AtomicBool>,
    stream: Arc<Mutex<Option<cpal::Stream>>>,
}

impl MusicPlayer {
    pub fn new() -> Self {
        MusicPlayer {
            music_playing: Arc::new(AtomicBool::new(false)), // Start as stopped
            stream: Arc::new(Mutex::new(None)),              // No stream initially
        }
    }

    pub fn play(&self) {
        let mut stream_lock = self.stream.lock().unwrap();

        // If already playing, do nothing
        if stream_lock.is_some() {
            return;
        }

        let music_playing = Arc::clone(&self.music_playing);
        music_playing.store(true, Ordering::SeqCst);

        let stream = play_sound(music_playing.clone()).expect("Failed to create audio stream");
        *stream_lock = Some(stream);
    }

    pub fn toggle(&self) {
        let currently_playing = self.music_playing.load(Ordering::SeqCst);
        self.music_playing
            .store(!currently_playing, Ordering::SeqCst);
        println!(
            "Music is now {}.",
            if currently_playing {
                "paused"
            } else {
                "playing"
            }
        );
    }

    pub fn stop(&self) {
        let mut stream_lock = self.stream.lock().unwrap();
        *stream_lock = None; // Drop the stream to stop playback
        self.music_playing.store(false, Ordering::SeqCst);
        println!("Music stopped.");
    }
}

fn play_sound(music_playing: Arc<AtomicBool>) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config().unwrap();
    let sample_rate = config.sample_rate().0 as f32;

    let shared_state = Arc::new(Mutex::new((0, 0, 0, 0, 0)));

    let stream = {
        let state = Arc::clone(&shared_state);

        device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                if !music_playing.load(Ordering::SeqCst) {
                    // Mute audio by setting samples to zero when paused
                    for sample in data.iter_mut() {
                        *sample = 0.0;
                    }
                    return;
                }

                let mut state = state.lock().unwrap();
                let (
                    mut melody_index,
                    mut bass_index,
                    mut melody_counter,
                    mut bass_counter,
                    mut sample_counter,
                ) = *state;

                let melody_duration_samples =
                    ((misty::MELODY[melody_index].1 * sample_rate as u64) / 1000) as u64;
                let bass_duration_samples =
                    ((misty::BASSLINE[bass_index].1 * sample_rate as u64) / 1000) as u64;

                let melody_volume = 0.2;
                let bass_volume = 0.1;

                for sample in data.iter_mut() {
                    if melody_counter as u64 >= melody_duration_samples {
                        melody_index = (melody_index + 1) % misty::MELODY.len();
                        melody_counter = 0;
                    }
                    if bass_counter as u64 >= bass_duration_samples {
                        bass_index = (bass_index + 1) % misty::BASSLINE.len();
                        bass_counter = 0;
                    }

                    let melody_freq = note_freq(misty::MELODY[melody_index].0);
                    let bass_freq = note_freq(misty::BASSLINE[bass_index].0);

                    let melody_wave =
                        generate_pulse_wave(sample_counter, melody_freq, sample_rate, 0.45);
                    let bass_wave = generate_triangle_wave(sample_counter, bass_freq, sample_rate);

                    *sample = (melody_wave * melody_volume) + (bass_wave * bass_volume);

                    melody_counter += 1;
                    bass_counter += 1;
                    sample_counter += 1;
                }

                *state = (
                    melody_index,
                    bass_index,
                    melody_counter,
                    bass_counter,
                    sample_counter,
                );
            },
            |err| eprintln!("Error: {}", err),
            None,
        )?
    };

    stream.play()?;
    Ok(stream)
}
