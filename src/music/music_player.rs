use crate::music::misty;
use crate::music::synth::{generate_pulse_wave, generate_triangle_wave, note_freq};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct MusicPlayer {
    music_playing: Arc<AtomicBool>,
    running: Arc<AtomicBool>,
}

impl MusicPlayer {
    pub fn new() -> Self {
        MusicPlayer {
            music_playing: Arc::new(AtomicBool::new(true)),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start_music_thread(&self) {
        let music_playing = Arc::clone(&self.music_playing);
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                if music_playing.load(Ordering::SeqCst) {
                    // println!("🎵 Playing sound...");
                    let _res = play_sound();
                } else {
                    println!("Music paused...");
                    thread::sleep(Duration::from_millis(100)); // Sleep when music is off
                }
            }
            println!("Music thread has exited.");
        });
    }

    pub fn toggle_music(&self) {
        let currently_playing = self.music_playing.load(Ordering::SeqCst);
        self.music_playing
            .store(!currently_playing, Ordering::SeqCst);
        println!(
            "Music is now {}.",
            if currently_playing {
                "stopped"
            } else {
                "playing"
            }
        );
    }

    pub fn stop_music_thread(&self) {
        self.running.store(false, Ordering::SeqCst); // Stop the music thread
        self.music_playing.store(false, Ordering::SeqCst); // Stop playing music
    }

    // FIXME: See if there's a better way to define the cpal Stream with the constructor
    //
    // Then maybe I can figure out how to toggle, change tracks, etc.
    //
    // pub fn play(&self) {
    //     if let Some(stream) = &self.stream {
    //         stream.play().expect("Failed to play stream");
    //     }
    // }
}

pub fn play_sound() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config().unwrap();
    let sample_rate = config.sample_rate().0 as f32;

    // (melody index, bass index, melody sample counter, bass sample counter, global sample counter)
    let shared_state = Arc::new(Mutex::new((0, 0, 0, 0, 0)));

    let stream = {
        let state = Arc::clone(&shared_state);

        device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut state = state.lock().unwrap();
                    let (
                        mut melody_index,
                        mut bass_index,
                        mut melody_counter,
                        mut bass_counter,
                        mut sample_counter,
                    ) = *state;

                    // Use integer division and round to make sure melody and bass don't get out of sync
                    let melody_duration_samples =
                        ((misty::MELODY[melody_index].1 * sample_rate as u64) / 1000) as u64;

                    // Use integer division and round to make sure melody and bass don't get out of sync
                    let bass_duration_samples =
                        ((misty::BASSLINE[bass_index].1 * sample_rate as u64) / 1000) as u64;

                    // Define volumes (adjust these as needed)
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

                        let bass_wave =
                            generate_triangle_wave(sample_counter, bass_freq, sample_rate);

                        // **Now control the mix, using the volume settings**
                        *sample = (melody_wave * melody_volume) + (bass_wave * bass_volume);

                        // Increment counters
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
            )
            .unwrap()
    };

    stream.play().unwrap();
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}
