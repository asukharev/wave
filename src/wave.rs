use std::f32::consts::PI;
use std::default::Default;

#[derive(Debug)]
pub struct Wave {
    name: String,
    pub sample_rate: u32,
    frequency: u32,
    amplitude: f32,
    duration: f32,
    pub data: Vec<f32>,
}

impl Wave {
    pub fn sin(name: &str, sample_rate: u32, frequency: u32, amplitude: f32, duration: f32) -> Wave {
        let samples_count = (sample_rate as f32 * duration) as u32;
        let samples: Vec<f32> = (0..samples_count).map(|x| {
            amplitude * (2.0f32 * PI * frequency as f32 * x as f32 / sample_rate as f32).sin()
        })
        .collect();
        Wave {
            name: name.to_string(),
            sample_rate: sample_rate,
            frequency: frequency,
            amplitude: amplitude,
            duration: duration,
            data: samples,
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Wave {
            name: "default".to_string(),
            sample_rate: 0,
            frequency: 0,
            amplitude: 0.0f32,
            duration: 0.0f32,
            data: Vec::new(),
        }
    }
}
