pub enum FilterType {
    Trashold(f32),
    Gaussian(f32),
    Convolution(Vec<f32>),
    Echo(usize, f32),
    Blur(usize),
    FFT,
    Reverb(usize, f32),
    Distortion(f32),
    None,
}