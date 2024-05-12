use crate::enums;

use num_complex::Complex;
use enums::filter_type::FilterType;

pub fn process_samples(samples: Vec<f32>, filter_type: FilterType) -> Vec<f32> {
    match filter_type {
        FilterType::Trashold(value) => trashold(samples, value),
        FilterType::Gaussian(sigma) => gaussian_filter(samples, sigma),
        FilterType::Convolution(kernel) => convolution(samples, kernel),
        FilterType::Echo(delay, decay) => echo(samples, delay, decay),
        FilterType::Blur(radius) => blur(samples, radius),
        FilterType::FFT => fft(samples),
        FilterType::Reverb(delay, decay) => reverb(samples, delay, decay),
        FilterType::Distortion(gain) => distortion(samples, gain),
        FilterType::None => samples,
    }
}

pub fn trashold(samples: Vec<f32>, trashold: f32) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        if samples[i] > trashold {
            new_samples[i] = samples[i];
        } else {
            new_samples[i] = 0.0;
        }
    }

    new_samples
}

pub fn gaussian_filter(samples: Vec<f32>, sigma: f32) -> Vec<f32> {
    let mut new_samples = samples.clone();
    let mut kernel = vec![0.0; 100];
    let mut sum = 0.0;
    for i in 0..100 {
        kernel[i] = (1.0 / (2.0 * 3.1415 * sigma * sigma)).exp()
            * (-((i as f32 - 50.0) * (i as f32 - 50.0) / (2.0 * sigma * sigma)));
        sum += kernel[i];
    }

    for i in 0..100 {
        kernel[i] /= sum;
    }

    for i in 50..samples.len() - 50 {
        let mut new_sample = 0.0;
        for j in 0..100 {
            new_sample += samples[i - 50 + j] * kernel[j];
        }
        new_samples[i] = new_sample;
    }

    new_samples
}

pub fn convolution(samples: Vec<f32>, kernel: Vec<f32>) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        let mut new_sample = 0.0;
        for j in 0..kernel.len() {
            if i >= j {
                new_sample += samples[i - j] * kernel[j];
            }
        }
        new_samples[i] = new_sample;
    }

    new_samples
}

pub fn echo(samples: Vec<f32>, delay: usize, decay: f32) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        if i >= delay {
            new_samples[i] = samples[i] + samples[i - delay] * decay;
        }
    }

    new_samples
}

pub fn blur(samples: Vec<f32>, radius: usize) -> Vec<f32> {
    let mut new_samples = samples.clone();
    let mut kernel = vec![0.0; 2 * radius + 1];
    let mut sum = 0.0;
    for i in 0..2 * radius + 1 {
        kernel[i] = 1.0;
        sum += kernel[i];
    }

    for i in 0..2 * radius + 1 {
        kernel[i] /= sum;
    }

    for i in radius..samples.len() - radius {
        let mut new_sample = 0.0;
        for j in 0..2 * radius + 1 {
            new_sample += samples[i - radius + j] * kernel[j];
        }
        new_samples[i] = new_sample;
    }

    new_samples
}

pub fn fft(samples: Vec<f32>) -> Vec<f32> {
    let mut new_samples = samples.clone();
    let mut planner = rustfft::FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut complex_samples: Vec<Complex<f32>> = samples
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();
    fft.process(&mut complex_samples);

    for i in 0..samples.len() {
        new_samples[i] = complex_samples[i].re;
    }

    new_samples
}

pub fn reverb(samples: Vec<f32>, delay: usize, decay: f32) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        if i >= delay {
            new_samples[i] = samples[i] + samples[i - delay] * decay;
        }
    }

    new_samples
}

pub fn distortion(samples: Vec<f32>, gain: f32) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        new_samples[i] = samples[i] * gain;
    }

    new_samples
}

pub fn seperate_channels(samples: Vec<f32>, channels: usize) -> Vec<Vec<f32>> {
    let mut new_samples = Vec::new();
    for _ in 0..channels {
        new_samples.push(Vec::new());
    }

    for i in 0..samples.len() {
        new_samples[i % channels].push(samples[i]);
    }

    new_samples
}

pub fn combine_channels(samples: Vec<Vec<f32>>) -> Vec<f32> {
    let mut new_samples = Vec::new();
    for i in 0..samples[0].len() {
        for j in 0..samples.len() {
            new_samples.push(samples[j][i]);
        }
    }

    new_samples
}