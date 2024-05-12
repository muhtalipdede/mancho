use hound::WavReader;

pub fn read_samples_from_file(filename: &str) -> Vec<f32> {
    let mut reader = WavReader::open(filename).unwrap();
    let samples: Vec<_> = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();
    samples
}

pub fn generate_samples(length : usize) -> Vec<f32> {
    let new_length = (length as f32 * 100000.0) as usize;
    let min = -200.0;
    let max = 200.0;
    let mut samples = Vec::new();
    let mut sample = 0.0;
    for i in 0..new_length {
        // harmonic wave
        // let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin() * 1000.0;

        // square wave
        // let sample = if (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin() > 0.0 { 1000.0 } else { -1000.0 };

        // sawtooth wave
        // let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).rem_euclid(2.0) * 1000.0 - 1000.0;

        // white noise
        // let sample = min + (max - min) * rand::random::<f32>();

        // pink noise
        // let white = min + (max - min) * rand::random::<f32>();
        // sample = 0.99886 * sample + white * 0.0555179;

        // brown noise
        // let white = min + (max - min) * rand::random::<f32>();
        // sample = 0.99 * sample + white * 0.1;

        // brown noise and pink noise
        // let white = min + (max - min) * rand::random::<f32>();
        // sample = 0.99 * sample + white * 0.1;
        // sample = 0.99886 * sample + white * 0.0555179;

        // bird sound
        // let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin() * 1000.0;
        samples.push(sample);
    }
    samples
}