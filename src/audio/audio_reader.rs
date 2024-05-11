use hound::WavReader;

pub fn read_samples_from_file(filename: &str) -> Vec<f32> {
    let mut reader = WavReader::open(filename).unwrap();
    let samples: Vec<_> = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();
    samples
}