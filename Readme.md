# Audio Processing in Rust

This project is a collection of audio processing functions written in Rust. It uses the `rustfft`, `hound`, and `image` crates for processing and manipulating audio data.

## Features

- Filtering: Removes all negative values from the audio samples.
- Thresholding: Sets all values below a certain threshold to zero.
- Gaussian Filter: Applies a Gaussian filter to the audio samples.
- Convolution: Applies a convolution operation on the audio samples with a given kernel.
- Echo: Adds an echo effect to the audio samples.
- Blur: Applies a blur effect to the audio samples.
- FFT (Fast Fourier Transform): Transforms the audio samples from the time domain to the frequency domain.

## Usage

To use this project, you need to have Rust installed on your machine. If you don't have Rust installed, you can install it from the [official website](https://www.rust-lang.org/tools/install).

1. Clone this repository.
2. Navigate to the project directory.
3. Run `cargo build` to build the project.
4. Run `cargo run` to execute the project.

![Original Audio](/src/assets/the-light-of-darkness.wav)
![Filtered Audio](/src/assets/the-light-of-darkness-filtered.wav)

## Images

![Original Audio](/src/assets/the-light-of-darkness.png)
![Blur Audio](/src/assets/the-light-of-darkness-blur.png)
![Echo Audio](/src/assets/the-light-of-darkness-echo.png)
![Convolution Audio](/src/assets/the-light-of-darkness-convolution.png)
![Gaussian Audio](/src/assets/the-light-of-darkness-gaussian_filter.png)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.