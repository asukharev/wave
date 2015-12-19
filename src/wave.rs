// http://soundfile.sapp.org/doc/WaveFormat/

use std::f32::consts::PI;

pub struct Wave(pub Vec<u8>);

pub fn sin(sample_rate: u32, duration: f32) -> Vec<f32> {
    let amplitude: f32 = 0.5f32;
    let frequency: f32 = 2000.0;

    let d = (sample_rate as f32 * duration) as u32;
    println!("{:?} = {:?} * {:?}", d, sample_rate, duration);

    let buffer: Vec<f32> = (0..d).map(|x| {
        amplitude * (2.0f32 * PI * frequency * x as f32 / sample_rate as f32).sin()
    })
    // .inspect(|y| {
    //     // println!("{} => {:1.6}", y, y);
    //     let b = y.to_binary();
    //     println!("{:1.10}\t=> {:32b} => {:x}", y, b, b);
    // })
    .collect();

    buffer
}
