struct CSVManager {}

struct WAVManager {
    sampling_rate: u32,
    channel_count: u16,
    bits_per_sample: u16,
}

impl WAVManager {
    fn generate_wav(&self) -> std::io::Result<()> {
        use std::f32::consts::PI;
        use std::fs::File;
        use std::path::Path;
        // const SAMPLING_RATE: u32 = 44100;
        let header = wav::Header::new(
            wav::header::WAV_FORMAT_PCM,
            self.channel_count,
            self.sampling_rate,
            self.bits_per_sample,
        );
        let mut vec = Vec::with_capacity(self.sampling_rate as usize);
        for i in 0..self.sampling_rate {
            let v =
                (i as f32 * 440.0 * 2.0 * PI / self.sampling_rate as f32).sin() * i16::MAX as f32;
            vec.push(v as i16);
        }
        let data: wav::BitDepth = wav::BitDepth::Sixteen(vec);
        let mut out_file =
            File::create(Path::new("wav/output.wav")).expect("Unable to create file");
        wav::write(header, &data, &mut out_file).expect("Unable to write file");
        Ok(())
    }
}

pub struct CSVtoWAV {
    csv_manager: CSVManager,
    wav_manager: WAVManager,
}

impl CSVtoWAV {
    pub fn new() -> Self {
        Self {
            csv_manager: CSVManager {},
            wav_manager: WAVManager {
                sampling_rate: 44100,
                channel_count: 1,
                bits_per_sample: 16,
            },
        }
    }
    pub fn transform(self) -> std::io::Result<()> {
        self.wav_manager.generate_wav()
    }
}
