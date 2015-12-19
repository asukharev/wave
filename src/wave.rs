use std::f32::consts::PI;

pub struct Wave(pub Vec<u8>);

pub fn sin(sample_rate: u32, frequency: u32, amplitude: f32, duration: f32) -> Vec<f32> {
    let samples_count = (sample_rate as f32 * duration) as u32;
    (0..samples_count).map(|x| {
        amplitude * (2.0f32 * PI * frequency as f32 * x as f32 / sample_rate as f32).sin()
    })
    .collect()
}
