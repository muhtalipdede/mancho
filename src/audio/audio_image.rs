use image::ImageBuffer;

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