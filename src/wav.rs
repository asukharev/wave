use std::fs::File;
use std::io::Write;
use std::convert::From;

use ::wave::Wave;

use ::vec_ext::{VecExt};

#[derive(Debug)]
enum Format {
    PCM = 0x0001, // WAVE_FORMAT_PCM     PCM
                 // 0x0003 	WAVE_FORMAT_IEEE_FLOAT 	IEEE float
                 // 0x0006 	WAVE_FORMAT_ALAW 	8-bit ITU-T G.711 A-law
                 // 0x0007 	WAVE_FORMAT_MULAW 	8-bit ITU-T G.711 µ-law
                 // 0xFFFE 	WAVE_FORMAT_EXTENSIBLE 	Determined by SubFormat
}

#[derive(Debug)]
pub struct Wav {
    group_id: String,
    file_length: u32,
    riff_type: String,

    // [fmt]
    fmt_group_id: String,
    fmt_chunk_size: u32,
    ftm_format_tag: u16,
    ftm_channels: u16,
    ftm_samples_per_sec: u32,
    ftm_avg_bytes_per_sec: u32,
    ftm_block_align: u16,
    ftm_bits_per_sample: u16,

    // [data]
    data_group_id: String,
    data_chunk_size: u32,
    data_samples: Vec<u8>
}

impl Wav {
    pub fn new() -> Self {
        Wav {
            group_id: "RIFF".to_string(),
            file_length: 44,
            riff_type: "WAVE".to_string(),

            fmt_group_id: "fmt ".to_string(),
            fmt_chunk_size: 16,
            ftm_format_tag: 0x0001,
            ftm_channels: 0,
            ftm_samples_per_sec: 0,
            ftm_avg_bytes_per_sec: 0,
            ftm_block_align: 0,
            ftm_bits_per_sample: 0,

            data_group_id: "data".to_string(),
            data_chunk_size: 0,
            data_samples: Vec::new()
        }
    }

    pub fn add_data(&mut self, wave: &Wave) {
        self.ftm_channels = 1;
        self.ftm_samples_per_sec = wave.sample_rate;
        self.ftm_bits_per_sample = 16;
        self.ftm_avg_bytes_per_sec =
            wave.sample_rate as u32 * self.ftm_channels as u32 * self.ftm_bits_per_sample as u32 / 8;
        self.ftm_block_align = self.ftm_channels * self.ftm_bits_per_sample / 8;
        self.data_chunk_size =
            wave.data.len() as u32 * self.ftm_channels as u32 * self.ftm_bits_per_sample as u32 / 8;
        for i in 0..wave.data.len() {
            let sample: i16 = (wave.data[i] * 32768 as f32) as i16;
            self.data_samples.push_i16_le(sample);
        }
        self.file_length = 44 + self.data_samples.len() as u32;
    }

    pub fn save(&self) {
        let mut header: Vec<u8> = vec![];
        {
            // sGroupID
            header.push_all_ext(self.group_id.as_bytes());
            // dwFileLength
            header.push_u32_le(self.file_length);
            // sRiffType
            header.push_all_ext(self.riff_type.as_bytes());
        }
        let mut fmt: Vec<u8> = vec![];
        {
            // sGroupID
            fmt.push_all_ext(self.fmt_group_id.as_bytes());
            // dwChunkSize
            fmt.push_u32_le(self.fmt_chunk_size);
            // wFormatTag
            fmt.push_u16_le(self.ftm_format_tag); // 0x0001 WAVE_FORMAT_PCM PCM
            // wChannels
            fmt.push_u16_le(self.ftm_channels);
            // dwSamplesPerSec
            fmt.push_u32_le(self.ftm_samples_per_sec);
            // dwAvgBytesPerSec
            fmt.push_u32_le(self.ftm_avg_bytes_per_sec);
            // wBlockAlign
            fmt.push_u16_le(self.ftm_block_align);
            // dwBitsPerSample
            fmt.push_u16_le(self.ftm_bits_per_sample);
        }
        let mut data: Vec<u8> = vec![];
        {
            // sGroupID
            data.push_all_ext(self.data_group_id.as_bytes());
            // dwChunkSize
            data.push_u32_le(self.data_chunk_size);
            // sampleData
            data.push_all_ext(&self.data_samples[..]);
        }

        let mut f: File = match File::create("wave.wav") {
            Ok(ff) => ff,
            Err(e) => panic!("{:?}", e),
        };

        let out: Vec<&Vec<u8>> = vec![&header, &fmt, &data];
        for vec in out {
            match f.write_all(&vec[..]) {
                Ok(_) => (),
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}

impl From<Wave> for Wav {
    fn from(wave: Wave) -> Self {
        let mut wav = Wav::new();
        wav.add_data(&wave);
        wav
    }
}
