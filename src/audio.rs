use rodio::{buffer::SamplesBuffer, Decoder, OutputStreamHandle, Source};
use std::{
    fs::File,
    io::BufReader,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

#[derive(Clone)]
pub struct SoundPlayer {
    handle: OutputStreamHandle,
    samples: Arc<Vec<f32>>,
    sample_rate: u32,
    channels: u16,
    volume: Arc<AtomicU32>,
}

impl SoundPlayer {
    pub fn new(file_path: &str, handle: OutputStreamHandle) -> Self {
        let file = File::open(file_path).expect("Failed to open sound file");
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).expect("Failed to decode sound file");

        let sample_rate = decoder.sample_rate();
        let channels = decoder.channels();
        let samples: Vec<f32> = decoder.convert_samples().collect();

        Self {
            handle,
            samples: Arc::new(samples),
            sample_rate,
            channels,
            volume: Arc::new(AtomicU32::new(100)),
        }
    }

    pub fn volume(&self) -> u32 {
        self.volume.load(Ordering::Relaxed)
    }

    pub fn set_volume(&self, vol: u32) {
        self.volume.store(vol.min(100), Ordering::Relaxed);
    }

    pub fn play_clip(&self, start_ms: u32, duration_ms: u32) {
        let samples_per_ms = (self.sample_rate as f64) / 1000.0 * (self.channels as f64);
        let mut start_idx = (start_ms as f64 * samples_per_ms) as usize;
        start_idx = start_idx.saturating_sub(start_idx % self.channels as usize);

        let mut end_idx = start_idx + (duration_ms as f64 * samples_per_ms) as usize;
        end_idx = end_idx.saturating_sub(end_idx % self.channels as usize);

        let len = self.samples.len();
        let start_idx = start_idx.min(len);
        let end_idx = end_idx.min(len);

        if start_idx < end_idx {
            let volume_factor = self.volume.load(Ordering::Relaxed) as f32 / 100.0;
            let slice: Vec<f32> = self.samples[start_idx..end_idx]
                .iter()
                .map(|&s| s * volume_factor)
                .collect();
            let source = SamplesBuffer::new(self.channels, self.sample_rate, slice);
            let _ = self.handle.play_raw(source);
        }
    }
}