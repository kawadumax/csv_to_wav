struct CSVManager {}

struct WAVManager {}
pub struct CSVtoWAV {
    csv_manager: CSVManager,
    wav_manager: WAVManager,
}

impl CSVtoWAV {
    pub fn transform() -> std::io::Result<()> {
        use std::f32::consts::PI;
        use std::fs::File;
        use std::path::Path;
        const SAMPLING_RATE: u32 = 44100;
        let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 1, SAMPLING_RATE, 16);
        let mut vec = Vec::with_capacity(SAMPLING_RATE as usize);
        for i in 0..SAMPLING_RATE {
            let v = (i as f32 * 440.0 * 2.0 * PI / SAMPLING_RATE as f32).sin() * i16::MAX as f32;
            vec.push(v as i16);
        }
        let data: wav::BitDepth = wav::BitDepth::Sixteen(vec);
        let mut out_file =
            File::create(Path::new("wav/output.wav")).expect("Unable to create file");
        wav::write(header, &data, &mut out_file).expect("Unable to write file");
        Ok(())
    }
}
