use image::RgbImage;
use image::math::utils::clamp;
use std::f64::consts::PI;
use std::f64::NAN;

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

pub fn gaussian_blur(img: &mut RgbImage) -> &mut RgbImage {
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

pub fn sobel(img: &mut RgbImage) -> (&mut RgbImage, Vec<Vec<f64>>) {
    let img_clone = img.clone();
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
                    let x =  clamp(j + k - 1, 0, width - 1);
                    let y = clamp(i + q - 1, 0, height - 1);
                    let pixel = img_clone.get_pixel(x as u32, y as u32).data[0] as i32;
                    g_x += pixel * horizontal_kernel[k as usize][q as usize];
                    g_y += pixel * vertical_kernel[k as usize][q as usize];
                }
            }
            let g_x = clamp(g_x, 0, 255);
            let g_y = clamp(g_y, 0, 255);
            let g = clamp_plain_color((g_x as f64).hypot(g_y as f64));
            img.put_pixel(j as u32, i as u32, g);
            let theta = ((g_y as f64).atan2(g_x as f64) * 4.0 / PI).round() * PI / 4.0 - PI / 2.0;
            directions[i as usize][j as usize] = if g.data[0] == 0 { NAN } else { theta };
        }
    }
    (img, directions)
}

pub fn non_maximum_suppression(img: &mut RgbImage, directions: Vec<Vec<f64>>) -> &mut RgbImage {
    let img_clone = img.clone();
    let width = img.width() as i32;
    let height = img.height() as i32;

    for i in 0..height as i32 {
        for j in 0..width as i32 {
            if directions[i as usize][j as usize].is_nan() {
                img.put_pixel(j as u32, i as u32, image::Rgb([0; 3]));
                continue
            }
            let dx = directions[i as usize][j as usize].cos().signum() as i32;
            let dy = directions[i as usize][j as usize].sin().signum() as i32;
            if j + dx >= 0 && i + dy >= 0 && j + dx < width && i + dy < height &&
                img_clone.get_pixel((j + dx) as u32, (i + dy) as u32).data[0] <= img_clone.get_pixel(j as u32, i as u32).data[0] {

                img.put_pixel((j + dx) as u32, (i + dy) as u32, image::Rgb([0; 3]));
            }
            if j - dx >= 0 &&
                i - dy >= 0 &&
                j - dx < width &&
                i - dy < height &&
                img_clone.get_pixel((j - dx) as u32, (i - dy) as u32).data[0] <= img_clone.get_pixel(j as u32, i as u32).data[0] {

                img.put_pixel((j - dx) as u32, (i - dy) as u32, image::Rgb([0; 3]));
            }
            img.put_pixel(j as u32, i as u32, img_clone.get_pixel(j as u32, i as u32).clone());
        }
    }
    img
}

pub fn threshold(img: &mut RgbImage, lower_bound: f64, upper_bound: f64) -> &mut RgbImage {
    let lower_bound = lower_bound * 255.0;
    let upper_bound = upper_bound * 255.0;
    let (width, height) = img.dimensions();
    for i in 0..height {
        for j in 0..width {
            let pixel = img.get_pixel(j, i).data[0] as f64;
            let new_pixel: image::Rgb<u8>;
            if pixel >= upper_bound {
                new_pixel = image::Rgb([255; 3]);
            } else if pixel < lower_bound {
                new_pixel = image::Rgb([0; 3]);
            } else {
                new_pixel = image::Rgb([127; 3]);
            }
            img.put_pixel(j, i, new_pixel);
        }
    }
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
                    let x = clamp(j + u, 0, width - 1);
                    let y = clamp(i + v, 0, height - 1);
                    for k in 0..3 {
                        sums[k] +=
                            kernel[(matrix_size / 2 + u) as usize][(matrix_size / 2 + v) as usize] as f64 *
                            img_clone.get_pixel(x as u32, y as u32).data[k] as f64 / divisor as f64;
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
    let clamped_value = clamp(value.round(), 0.0, 255.0) as u8;
    let values = [clamped_value; 3];
    image::Rgb(values)
}