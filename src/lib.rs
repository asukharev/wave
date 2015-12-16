use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn wave(sample_rate: u32, len: u32) -> Vec<f32> {
    let mut buffer: Vec<f32> = Vec::new();
    let amplitude: u8 = (0.25 * 255.0) as u8; // short.MaxValue;
    let frequency = 1000.0;
    for n in 0..sample_rate {
        let v =
            amplitude as f32 *
            (
                (2.0 * PI * n as f32 * frequency as f32)
                / sample_rate as f32
            ).sin();
        buffer.push(v);
    }
    buffer

    // let mut b: Vec<u8> = Vec::new();
    // for c in buffer.iter() {
    // //     print!("{:?}", c);
    //
    //     let mut cc: [u8; 4] = [0; 4];
    //     cc[0] = (((*c as u32) <<  0) >> 24) as u8;
    //     cc[1] = (((*c as u32) <<  8) >> 24) as u8;
    //     cc[2] = (((*c as u32) << 16) >> 24) as u8;
    //     cc[3] = (((*c as u32) << 24) >> 24) as u8;
    //     b.push(cc[0]);
    //     b.push(cc[1]);
    //     b.push(cc[2]);
    //     b.push(cc[3]);
    //
    //     println!("{:?}", c);
    //     println!("{:?}", cc);
    // }

    // let mut f: std::fs::File = match File::create("out.wav") {
    //     Ok(ff) => ff,
    //     Err(e) => panic!("{:?}", e),
    // };

    // match f.write_all(&b[..]) {
    //     Ok(_) => println!("OK"),
    //     Err(e) => panic!("{:?}", e),
    // }
}

fn push_all(o: &mut Vec<u8>, src: &[u8]) {
    for b in src {
        o.push(b.clone());
    }
}

fn push_u32_le(o: &mut Vec<u8>, c: u32) {
    let mut cc: [u8; 4] = [0; 4];
    cc[0] = (((c as u32) <<  0) >> 24) as u8;
    cc[1] = (((c as u32) <<  8) >> 24) as u8;
    cc[2] = (((c as u32) << 16) >> 24) as u8;
    cc[3] = (((c as u32) << 24) >> 24) as u8;
    o.push(cc[3]);
    o.push(cc[2]);
    o.push(cc[1]);
    o.push(cc[0]);
}

fn push_u32_be(o: &mut Vec<u8>, c: u32) {
    let mut cc: [u8; 4] = [0; 4];
    cc[0] = (((c as u32) <<  0) >> 24) as u8;
    cc[1] = (((c as u32) <<  8) >> 24) as u8;
    cc[2] = (((c as u32) << 16) >> 24) as u8;
    cc[3] = (((c as u32) << 24) >> 24) as u8;
    o.push(cc[0]);
    o.push(cc[1]);
    o.push(cc[2]);
    o.push(cc[3]);
}

fn push_u16(o: &mut Vec<u8>, c: u16) {
    let mut cc: [u8; 2] = [0; 2];
    cc[0] = (((c as u32) <<  0) >> 8) as u8;
    cc[1] = (((c as u32) <<  8) >> 8) as u8;
    o.push(cc[1]);
    o.push(cc[0]);
}

// fn f32_to_u8<'a>() -> &'a [u8] {
//
// }

#[test]
fn it_works() {
    let samples_rate: u32 = 8000;
    let channels: u16 = 1;
    let bits_per_sample: u16 = 32;
    let avg_bytes_per_sec: u32 = bits_per_sample as u32 / 8 * 2;
    let duration: u32 = 0.1 as u32;

    let wave = wave(samples_rate, duration);
    println!("{:?}", wave.len());

    let mut header: Vec<u8> = vec![];
    {
        // sGroupID
        push_all(&mut header, "RIFF".as_bytes());
        // dwFileLength
        push_u32_le(&mut header, (44 + (wave.len() * 4)) as u32);
        // sRiffType
        push_all(&mut header, "WAVE".as_bytes());
    }
    let mut fmt: Vec<u8> = vec![];
    {
        // sGroupID
        push_all(&mut fmt, "fmt ".as_bytes());
        // dwChunkSize
        push_u32_le(&mut fmt, 16 as u32);
        // wFormatTag
        push_u16(&mut fmt, 1);
        // wChannels
        push_u16(&mut fmt, 1);
        // dwSamplesPerSec
        push_u32_le(&mut fmt, samples_rate);
        // dwAvgBytesPerSec
        push_u32_le(&mut fmt, avg_bytes_per_sec); // sampleRate * blockAlign
        // wBlockAlign
        push_u16(&mut fmt, channels * bits_per_sample / 8); // wChannels * (dwBitsPerSample / 8)
        // dwBitsPerSample
        push_u16(&mut fmt, bits_per_sample); // 16
    }
    let mut data: Vec<u8> = vec![];
    {
        // sGroupID
        push_all(&mut data, "data".as_bytes());
        // dwChunkSize
        let s = samples_rate * channels as u32 * duration;
        push_u32_le(&mut data, s as u32);
        // sampleData
        // dwSamplesPerSec * wChannels * duration of audio in seconds
        for i in 0..wave.len() {
            // println!("{:?}", wave[i]);
            push_u32_be(&mut data, wave[i] as u32);
            // data.push(wave[i]);
        }
    }




    let mut out: Vec<u8> = Vec::new();
    push_all(&mut out, &header);
    push_all(&mut out, &fmt);
    push_all(&mut out, &data);

    let mut f: std::fs::File = match File::create("out.wav") {
        Ok(ff) => ff,
        Err(e) => panic!("{:?}", e),
    };

    match f.write_all(&out[..]) {
        Ok(_) => println!("OK"),
        Err(e) => panic!("{:?}", e),
    }

    assert!(false);
}
