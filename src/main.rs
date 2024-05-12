extern crate hound;
extern crate image;
extern crate rustfft;

mod audio;
mod enums;

use audio::audio_processing::process_samples;
use audio::audio_writer::write_samples_to_file;
use enums::filter_type::FilterType;
use audio::audio_image::{wav2image, wav2histogram};

use crate::audio::audio_processing::seperate_channels;

fn main() {
    let filepath = "src/assets/bird1";
    let file_extension = ".wav";
    let filename = filepath.to_string() + file_extension;
    let reader = hound::WavReader::open(&filename).unwrap();


    // let samples = audio::audio_reader::generate_samples(10);
    let samples = audio::audio_reader::read_samples_from_file(&filename);
    // let channels = seperate_channels(samples.clone(), reader.spec().channels as usize);

    let filter_type = FilterType::Gaussian(0.1);

    let new_samples = process_samples(samples, filter_type);

    write_samples_to_file(&new_samples, &(filepath.to_string() + "-filtered.wav"), reader.spec());
    wav2image(new_samples.clone(), &(filepath.to_string() + "-filtered.png"));
    wav2histogram(new_samples.clone(), &(filepath.to_string() + "-histogram.png"));
    
    println!("Done!")
}