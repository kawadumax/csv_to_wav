use wav::BitDepth;

fn main() -> std::io::Result<()> {
    use std::f32::consts::PI;
    use std::fs::File;
    use std::path::Path;

    let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 2, 44_100, 16);
    let mut vec = Vec::with_capacity(44100);
    for i in 0..44100 {
        let v = (i as f32 * 440.0 * 2.0 * PI / 44100.0).sin() * i16::MAX as f32;
        vec.push(v as i16);
    }
    let data: BitDepth = wav::BitDepth::Sixteen(vec);
    let mut out_file = File::create(Path::new("wav/output.wav")).unwrap();
    wav::write(header, &data, &mut out_file).expect("Unable to write file");
    Ok(())
}
