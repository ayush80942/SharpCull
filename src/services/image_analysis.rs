use image::{DynamicImage, GenericImageView, Luma};
use image::imageops::grayscale;

pub fn analyze_blur(img: &DynamicImage) -> f32 {
    let gray = grayscale(img);
    let (width, height) = gray.dimensions();

    let mut sum = 0.0;
    let mut count = 0.0;

    // Laplacian filter
    let kernel: [[i32; 3]; 3] = [
        [0,  1, 0],
        [1, -4, 1],
        [0,  1, 0],
    ];

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut laplacian = 0.0;
            for ky in 0..3 {
                for kx in 0..3 {
                    let px = gray.get_pixel(x + kx - 1, y + ky - 1)[0] as f32;
                    laplacian += px * kernel[ky as usize][kx as usize] as f32;
                }
            }
            sum += laplacian.powf(2.0);
            count += 1.0;
        }
    }

    if count == 0.0 { return 0.0; }
    sum / count // Variance of Laplacian = blur score
}

pub fn analyze_brightness(img: &DynamicImage) -> f32 {
    let gray = grayscale(img);
    let (width, height) = gray.dimensions();
    let mut sum = 0.0;

    for y in 0..height {
        for x in 0..width {
            let pixel = gray.get_pixel(x, y)[0] as f32;
            sum += pixel;
        }
    }

    let total_pixels = (width * height) as f32;
    if total_pixels == 0.0 { return 0.0; }
    sum / total_pixels // Average intensity
}
