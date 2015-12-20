// http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html
// http://soundfile.sapp.org/doc/WaveFormat/

#![allow(dead_code)]

mod wav;
mod bits;
mod vec_ext;
mod wave;

#[test]
fn generate_wave() {
    use wave::Wave;

    let sample_rate = 8000;
    let duration = 1.0 as f32;
    let wave = Wave::sin("channel1", sample_rate, 2400, 1.0f32, duration);
    // println!("{:?}", wave);

    let wav = wav::Wav::from(wave);
    // println!("{:?}", wav);
    wav.save();

    assert!(false);
}
