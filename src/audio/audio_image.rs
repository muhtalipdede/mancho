use image::{ImageBuffer, Rgb};

pub fn wav2image(samples: Vec<f32>, filename: &str) {
    let mut imgbuf = ImageBuffer::new(800, 600);
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

pub fn wav2histogram(samples: Vec<f32>, filename: &str) {
    let mut histogram = vec![0; 256];

    let max = *samples.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
    let min = *samples.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
    let scale = 255.0 / (max - min + 1.0); // +1 eklendi

    for sample in samples {
        let value = ((sample - min) * scale) as usize;
        histogram[value] += 1;
    }

    let mut imgbuf = ImageBuffer::new(256, 100);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = histogram[x as usize] as f32; 
        if y < value as u32 {
            *pixel = Rgb([0u8, 0u8, 0u8]);
        } else {
            *pixel = Rgb([255u8, 255u8, 255u8]);
        }
    }
    imgbuf.save(filename).unwrap();
}