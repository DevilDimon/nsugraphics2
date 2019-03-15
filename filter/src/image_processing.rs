use image::RgbImage;
use image::math::utils::clamp;

pub fn greyscale(img: &mut RgbImage) -> &mut RgbImage {
    for pixel in img.pixels_mut() {
        let red = (pixel.data[0] as f64 * 0.299).round() as u8;
        let green = (pixel.data[1] as f64 * 0.587).round() as u8;
        let blue = (pixel.data[2] as f64 * 0.114).round() as u8;
        let sum = red + green + blue;
        pixel.data = [sum, sum, sum];
    }
    img
}

pub fn gaussian_blur(img: &mut RgbImage, sigma: f32) -> &RgbImage {
    let kernel = vec![
        vec![2, 4, 5, 4, 2],
        vec![4, 9, 12, 9, 4],
        vec![5, 12, 15, 12, 5],
        vec![4, 9, 12, 9, 4],
        vec![2, 4, 5, 4, 2]
    ];
    let divisor = 159;
    kernel_filter(img, kernel, divisor)
}

fn kernel_filter(img: &mut RgbImage, kernel: Vec<Vec<i32>>, divisor: i32) -> &RgbImage {
    let width = img.width() as i32;
    let height = img.height() as i32;
    let matrix_size = kernel.len() as i32;
    for i in 0..height as i32 {
        for j in 0..width as i32 {
            let mut sums = [0.0, 0.0, 0.0];
            for u in (-matrix_size / 2)..=(matrix_size / 2) {
                for v in (-matrix_size / 2)..=(matrix_size / 2) {
                    if j + u  < 0 || j + u > width - 1 || i + v < 0 || i + v > height - 1 {
                        continue;
                    }
                    for k in 0..3 {
                        sums[k] +=
                            kernel[(matrix_size / 2 + u) as usize][(matrix_size / 2 + v) as usize] as f64 *
                            img.get_pixel((j + u) as u32, (i + v) as u32).data[k] as f64 / divisor as f64;
                    }
                }
            }

            let mut values: [u8; 3] = [0, 0, 0];
            for k in 0..3 {
                values[k] = clamp(sums[k].round() as u8, 0, 0xFF);
            }

            img.put_pixel(j as u32, i as u32, image::Rgb(values));
        }
    }
    img
}