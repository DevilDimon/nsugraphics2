extern crate image;

use crate::image_processing::{greyscale, gaussian_blur};

mod image_details;
mod image_processing;

fn main() {
    let filename = std::env::args().nth(1)
        .expect("no path given");
    let mut img = image::open(std::path::Path::new(&filename))
        .expect("Cannot find the image at the given path")
        .to_rgb();

    println!("{}", image_details::extract_image_details(std::path::Path::new(&filename)
        .file_stem()
        .expect("Wrong filename")
        .to_str()
        .expect("Wrong filename")));

    let mut greyscaled = greyscale(&mut img);
    let mut blurres = gaussian_blur(&mut greyscaled, 0.0);
    blurres.save("result.jpg").expect("Failed to save");
}
