use chrono::{Date, Local};
use std::{io::SeekFrom, ops::Deref};
use wav::BitDepth;

struct CSVManager {
    data: Vec<i32>,
}

struct WAVManager {
    sampling_rate: u32,
    channel_count: u16,
    bits_per_sample: u16,
}

impl CSVManager {
    fn extract_data(&mut self) {
        println!("Extract price data from CSV");
        // // catでデータを直接読み込む場合
        // let mut rdr = csv::ReaderBuilder::new()
        //     .has_headers(false)
        //     .from_reader(std::io::stdin());
        let mut rdr = csv::ReaderBuilder::new()
            .from_path("csv/bitstampUSD.csv")
            .unwrap();
        let mut line_num;
        for result in rdr.records() {
            let record = result.expect("a CSV record");
            line_num = record.position().unwrap().line();
            if line_num % 100000 == 0 {
                println!("{}", line_num)
            }
            let record = record.get(1).unwrap();
            let data: f32 = record.parse().unwrap();
            self.data.push(data as i32)
            // self.data.push(data)
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
        // let date = chrono::Local::today();
        // let path = format!("wav/{}output.wav", date.to_string());
        let mut out_file = File::create(Path::new("output.wav")).expect("Unable to create file");
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
    pub fn transform(mut self) -> std::io::Result<()> {
        self.csv_manager.extract_data();
        let raw_data = self.csv_manager.data;
        self.wav_manager.generate_wav(Self::normalize(raw_data))
    }
    // raw_dataをwavの許す範囲に正規化する
    // raw_dataの最大値Max、最小値Minを、i16bitの最大値16BitMaxと最小値16BitMinにマッピングする
    // f(x) = (i16::MAX - i16::MIN)/(Max - Min)(x - Max) + i16::MAX
    fn normalize(raw_data: Vec<i32>) -> Vec<i16> {
        let min = *raw_data.iter().min().unwrap();
        let max = *raw_data.iter().max().unwrap();
        println!("start normalize");
        raw_data
            .iter()
            .map(|x| {
                ((i16::MAX as i32 - i16::MIN as i32) / (max - min) * (x - max) + i16::MAX as i32)
                    as i16
            })
            .collect()
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
    /// テスト用関数だけど、同時にCSV生成用の関数。
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
    ///CSVから適切にWAVに変換できるかどうかというもの。入力用のCSVには`generate_sine_csv`を想定しています。
    fn generate_sine_wav_from_csv() -> std::io::Result<()> {
        let mut rdr = csv::ReaderBuilder::new().from_path("csv/sine_data.csv")?;
        let mut raw_data = Vec::with_capacity(44100);
        for result in rdr.records() {
            let record = result.expect("a CSV record");
            let record = record.get(0).unwrap();
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
