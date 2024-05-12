extern crate hound;
extern crate image;
extern crate rustfft;

mod audio;
mod enums;

use audio::audio_image::wav2image;
use audio::audio_processing::process_samples;
use audio::audio_writer::write_samples_to_file;
use audio::audio_reader::read_samples_from_file;
use enums::filter_type::FilterType;

fn main() {
    let filename = "src/assets/the-light-of-darkness.wav";
    let reader = hound::WavReader::open(filename).unwrap();
    let samples = read_samples_from_file(filename);
    let filter_type = FilterType::Gaussian(10.0);

    let new_samples = process_samples(samples, filter_type);

    write_samples_to_file(&new_samples, "src/assets/the-light-of-darkness-filtered.wav", reader.spec());
    wav2image(new_samples, "src/assets/the-light-of-darkness-filtered.png");
    
    println!("Done!")
}