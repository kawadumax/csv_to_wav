fn main() {
    use std::fs::File;
    use std::path::Path;

    let mut inp_file = File::open(Path::new("data/sine.wav"))?;
    let (header, data) = wav::read(&mut inp_file)?;

    let mut out_file = File::create(Path::new("data/output.wav"))?;
    wav::write(header, &data, &mut out_file)?;
}
