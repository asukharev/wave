// http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html

#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
mod bits;
use bits::{Bits};
mod wave;
mod vec_ext;
use vec_ext::{VecExt};

// 0x0001 	WAVE_FORMAT_PCM 	PCM
// 0x0003 	WAVE_FORMAT_IEEE_FLOAT 	IEEE float
// 0x0006 	WAVE_FORMAT_ALAW 	8-bit ITU-T G.711 A-law
// 0x0007 	WAVE_FORMAT_MULAW 	8-bit ITU-T G.711 µ-law
// 0xFFFE 	WAVE_FORMAT_EXTENSIBLE 	Determined by SubFormat

#[test]
fn generate_wave() {
    let sample_rate: u32 = 44100;
    let channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let duration: f32 = 1.0 as f32;

    let wave = wave::sin(sample_rate, duration);

    let mut header: Vec<u8> = vec![];
    {
        // sGroupID
        header.push_all_ext("RIFF".as_bytes());
        // dwFileLength
        header.push_u32_le((44 + (wave.len() * 4)) as u32);
        // sRiffType
        header.push_all_ext("WAVE".as_bytes());
    }
    let mut fmt: Vec<u8> = vec![];
    {
        // sGroupID
        fmt.push_all_ext("fmt ".as_bytes());
        // dwChunkSize
        fmt.push_u32_le(16 as u32);
        // wFormatTag
        fmt.push_u16_le(1); // 0x0001 WAVE_FORMAT_PCM PCM
        // wChannels
        fmt.push_u16_le(1);
        // dwSamplesPerSec
        fmt.push_u32_le(sample_rate);
        // dwAvgBytesPerSec
        fmt.push_u32_le(sample_rate as u32 * channels as u32 * bits_per_sample as u32 / 8);
        // wBlockAlign
        fmt.push_u16_le(channels * bits_per_sample / 8);
        // dwBitsPerSample
        fmt.push_u16_le(bits_per_sample);
    }
    let mut data: Vec<u8> = vec![];
    {
        // sGroupID
        data.push_all_ext("data".as_bytes());
        // dwChunkSize
        let s = wave.len() as u32 * channels as u32 * bits_per_sample as u32 / 8;
        data.push_u32_le(s as u32);
        // sampleData
        for i in 0..wave.len() {
            // data.push_u32_le(wave[i].to_binary());
            let sample: i16 = (wave[i] * 32768 as f32) as i16;
            data.push_i16_le(sample);
        }
    }

    let mut out: Vec<u8> = Vec::new();
    out.push_all_ext(&header);
    out.push_all_ext(&fmt);
    out.push_all_ext(&data);

    let mut f: std::fs::File = match File::create("wave.wav") {
        Ok(ff) => ff,
        Err(e) => panic!("{:?}", e),
    };

    match f.write_all(&out[..]) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    assert!(false);
}
