extern crate hound;
extern crate image;
extern crate rustfft;

use num_complex::Complex;

fn filter(samples: Vec<f32>) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        if samples[i] > 0.0 {
            new_samples[i] = samples[i];
        } else {
            new_samples[i] = 0.0;
        }
    }

    new_samples
}

fn trashold(samples: Vec<f32>, trashold: f32) -> Vec<f32> {
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

fn gaussian_filter(samples: Vec<f32>, sigma: f32) -> Vec<f32> {
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

fn convolution(samples: Vec<f32>, kernel: Vec<f32>) -> Vec<f32> {
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

fn echo(samples: Vec<f32>, delay: usize, decay: f32) -> Vec<f32> {
    let mut new_samples = samples.clone();
    for i in 0..samples.len() {
        if i >= delay {
            new_samples[i] = samples[i] + samples[i - delay] * decay;
        }
    }

    new_samples
}

fn blur(samples: Vec<f32>, radius: usize) -> Vec<f32> {
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

fn fft(samples: Vec<f32>) -> Vec<f32> {
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

fn wav2image(samples: Vec<f32>, filename: &str) {
    let mut imgbuf = image::ImageBuffer::new(800, 600);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = (x + y * 800) as usize;
        if i < samples.len() {
            // Scale the sample to the 0-255 range
            let value = samples[i] as u8;
            *pixel = image::Rgb([value, value, value]);
        }
    }
    imgbuf.save(filename).unwrap();
}

fn main() {
    let filename = "src/assets/the-light-of-darkness.wav";

    let mut reader = hound::WavReader::open(filename).unwrap();
    let samples: Vec<_> = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();

    
    // let new_samples = trashold(samples, 30.0);
    let new_samples = gaussian_filter(samples, 10.0);
    // let new_samples = convolution(samples, vec![0.1, 0.2, 0.3, 0.2, 0.1]);
    // let new_samples = echo(samples, 10000, 2.0);
    // let new_samples = blur(samples, 100);
    // let new_samples = fft(samples);

    let mut writer = hound::WavWriter::create(
        "src/assets/the-light-of-darkness-filtered.wav",
        reader.spec(),
    )
    .unwrap();
    for sample in new_samples.iter() {
        writer.write_sample(*sample as i16).unwrap();
    }


    wav2image(new_samples, "src/assets/the-light-of-darkness-filtered.png");

    println!("Done!")
}
