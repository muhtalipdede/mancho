extern crate hound;
extern crate image;
extern crate rustfft;

mod audio;

use audio::audio_image::wav2image;
use audio::audio_processing::{trashold, gaussian_filter, convolution, echo, blur, fft};
use audio::filter_type::FilterType;
use audio::audio_writer::write_samples_to_file;
use audio::audio_reader::read_samples_from_file;

fn main() {
    let filename = "src/assets/the-light-of-darkness.wav";

    let reader = hound::WavReader::open(filename).unwrap();
    let samples = read_samples_from_file(filename);

    let filter_type = FilterType::Gaussian(10.0);

    let new_samples = match filter_type {
        FilterType::Trashold(value) => trashold(samples, value),
        FilterType::Gaussian(sigma) => gaussian_filter(samples, sigma),
        FilterType::Convolution(kernel) => convolution(samples, kernel),
        FilterType::Echo(delay, decay) => echo(samples, delay, decay),
        FilterType::Blur(radius) => blur(samples, radius),
        FilterType::FFT => fft(samples),
    };

    write_samples_to_file(&new_samples, "src/assets/the-light-of-darkness-filtered.wav", reader.spec());
    wav2image(new_samples, "src/assets/the-light-of-darkness-filtered.png");
    
    println!("Done!")
}
