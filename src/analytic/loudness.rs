use ebur128::{EbuR128, Mode};

#[derive(Debug)]
pub struct Meter {
    meter: EbuR128,
    channels: u32,
    #[allow(dead_code)]
    sample_rate: u32
}

impl Meter {
    pub fn new(channels: u32, sample_rate: u32) -> Self {
        let modes = Mode::I | Mode::S | Mode::TRUE_PEAK;
        let meter = EbuR128::new(channels, sample_rate, modes)
            .expect("Failed to create EBU R128 meter");
        Self {
            meter,
            channels,
            sample_rate
        }
    }

    pub fn add_frames_f32(&mut self, samples: &[f32]) {
        self.meter.add_frames_f32(samples).ok();
    }

    pub fn lufs_integrated(&self) -> Option<f64> {
        self.meter.loudness_global().ok()
    }

    pub fn lufs_shortterm(&self) -> Option<f64> {
        self.meter.loudness_shortterm().ok()
    }

    pub fn true_peaks(&self) -> Option<Vec<f64>> {
        let true_peaks: Option<Vec<f64>> = (0..self.channels)
            .map(|ch| self.meter.true_peak(ch as u32).ok())
            .collect::<Option<Vec<f64>>>();
        true_peaks
    }

}
