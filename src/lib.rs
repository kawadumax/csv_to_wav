use std::io::SeekFrom;

use wav::BitDepth;

struct CSVManager {
    data: Vec<i16>,
}

struct WAVManager {
    sampling_rate: u32,
    channel_count: u16,
    bits_per_sample: u16,
}

impl CSVManager {
    fn extract_data(&self) {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(std::io::stdin());
        for result in rdr.records() {
            let record = result.expect("a CSV record");
            // self.data.push(record)
        }
    }
}

impl WAVManager {
    fn generate_wav(&self, raw_data: Vec<i16>) -> std::io::Result<()> {
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
        let data: wav::BitDepth = wav::BitDepth::Sixteen(raw_data);
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
            csv_manager: CSVManager {
                data: Vec::with_capacity(55866758),
            },
            wav_manager: WAVManager {
                sampling_rate: 44100,
                channel_count: 1,
                bits_per_sample: 16,
            },
        }
    }
    pub fn transform(self) -> std::io::Result<()> {
        self.csv_manager.extract_data();
        let raw_data = self.csv_manager.data;
        self.wav_manager.generate_wav(raw_data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn generate_sine_wav() -> std::io::Result<()> {
        use std::f32::consts::PI;
        use std::fs::File;
        use std::path::Path;
        // const SAMPLING_RATE: u32 = 44100;
        let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 1, 44100, 16);
        let mut vec = Vec::with_capacity(44100);
        for i in 0..44100 {
            let v = (i as f32 * 440.0 * 2.0 * PI / 44100 as f32).sin() * i16::MAX as f32;
            vec.push(v as i16);
        }
        let data: wav::BitDepth = wav::BitDepth::Sixteen(vec);
        let mut out_file =
            File::create(Path::new("wav/sine_output.wav")).expect("Unable to create file");
        wav::write(header, &data, &mut out_file).expect("Unable to write file");
        Ok(())
    }

    #[test]
    fn generate_sine_csv() -> std::io::Result<()> {
        // create a file called data.csv, overwriting the file if it already existed
        // let mut csv = std::fs::File::create("csv/sine_data.csv");
        use std::f32::consts::PI;
        let mut wtr = csv::Writer::from_path("csv/sine_data.csv")?;

        for i in 0..44100 {
            let v_440 = (i as f32 * 440.0 * 2.0 * PI / 44100 as f32).sin() * i16::MAX as f32;
            let v_880 = (i as f32 * 880.0 * 2.0 * PI / 44100 as f32).sin() * i16::MAX as f32;
            wtr.write_record(&[v_440.to_string(), v_880.to_string()])?;
        }
        Ok(())
    }

    #[test]
    fn generate_sine_wav_from_csv() -> std::io::Result<()> {
        let mut rdr = csv::ReaderBuilder::new().from_path("csv/sine_data.csv")?;
        let mut raw_data = Vec::with_capacity(44100);
        for result in rdr.records() {
            let record = result.expect("a CSV record");
            // stringRecordをi16に変換したい
            // dbg!(record.to_owned());
            let record = record.get(0).unwrap();
            // dbg!(record);
            let sine_data: f32 = record.parse().unwrap();

            raw_data.push(sine_data as i16);
        }
        let data: wav::BitDepth = wav::BitDepth::Sixteen(raw_data);
        let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 1, 44100, 16);

        let mut out_file = std::fs::File::create(std::path::Path::new("wav/sine_output.wav"))
            .expect("Unable to create file");
        wav::write(header, &data, &mut out_file).expect("Unable to write file");

        Ok(())
    }
}
