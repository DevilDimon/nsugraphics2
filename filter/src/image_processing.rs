use image::RgbImage;
use image::math::utils::clamp;
use std::f64::consts::PI;

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

pub fn gaussian_blur(img: &mut RgbImage, sigma: f32) -> &mut RgbImage {
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

pub fn sobel_non_maximum_suppressed(img: &mut RgbImage) -> &mut RgbImage {
    let mut img_clone = img.clone();
    let horizontal_kernel = vec![
        vec![1, 2, 1],
        vec![0, 0, 0],
        vec![-1, -2, -1]
    ];
    let vertical_kernel = vec![
        vec![1, 0, -1],
        vec![2, 0, -2],
        vec![1, 0, -1]
    ];
    let width = img.width() as i32;
    let height = img.height() as i32;
    let mut directions = vec![vec![0.0; width as usize]; height as usize];

    for i in 0..height {
        for j in 0..width {
            let mut g_x = 0;
            let mut g_y = 0;

            for k in 0..3 {
                for q in 0..3 {
                    let x =  if j + k - 1 < width - 1 && j + k - 1 >= 0 { j + k - 1  } else { j };
                    let y = if i + q - 1 < height - 1 && i + q - 1 >= 0 { i + q - 1 } else { i };
                    let pixel = img_clone.get_pixel(x as u32, y as u32).data[0] as i32;
                    g_x += pixel * horizontal_kernel[k as usize][q as usize];
                    g_y += pixel * vertical_kernel[k as usize][q as usize];
                }
            }
            let g = clamp_plain_color((g_x as f64).hypot(g_y as f64));
            img.put_pixel(j as u32, i as u32, g);
            let theta = ((g_y as f64).atan2(g_x as f64) * 4.0 / PI).round() * PI / 4.0 - PI / 2.0;
            directions[i as usize][j as usize] = theta;
        }
    }

//    for i in 0..height {
//        for j in 0..width {
//
//        }
//    }

    img
}

fn kernel_filter(img: &mut RgbImage, kernel: Vec<Vec<i32>>, divisor: i32) -> &mut RgbImage {
    let width = img.width() as i32;
    let height = img.height() as i32;
    let matrix_size = kernel.len() as i32;
    let img_clone = img.clone();
    for i in 0..height {
        for j in 0..width {
            let mut sums = [0.0, 0.0, 0.0];
            for u in (-matrix_size / 2)..=(matrix_size / 2) {
                for v in (-matrix_size / 2)..=(matrix_size / 2) {
                    if j + u  < 0 || j + u > width - 1 || i + v < 0 || i + v > height - 1 {
                        continue;
                    }
                    for k in 0..3 {
                        sums[k] +=
                            kernel[(matrix_size / 2 + u) as usize][(matrix_size / 2 + v) as usize] as f64 *
                            img_clone.get_pixel((j + u) as u32, (i + v) as u32).data[k] as f64 / divisor as f64;
                    }
                }
            }

            let pixel = clamp_color(sums);
            img.put_pixel(j as u32, i as u32, pixel);
        }
    }
    img
}

fn clamp_color(components: [f64; 3]) -> image::Rgb<u8> {
    let mut values: [u8; 3] = [0, 0, 0];
    for k in 0..3 {
        values[k] = clamp(components[k].round() as u8, 0, 0xFF);
    }
    image::Rgb(values)
}

fn clamp_plain_color(value: f64) -> image::Rgb<u8> {
    let clamped_value = clamp(value.round() as u8, 0, 0xFF);
    let values: [u8; 3] = [clamped_value, clamped_value, clamped_value];
    image::Rgb(values)
}