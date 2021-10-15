use csv_to_wav::CSVtoWAV;
fn main() -> std::io::Result<()> {
    CSVtoWAV::new().transform()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
