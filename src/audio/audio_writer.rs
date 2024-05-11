use hound::WavWriter;
use hound::WavSpec;

pub fn write_samples_to_file(samples: &[f32], filename: &str, spec: WavSpec) {
    let mut writer = WavWriter::create(filename, spec).unwrap();
    for sample in samples.iter() {
        writer.write_sample(*sample as i16).unwrap();
    }
}