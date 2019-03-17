extern crate image;

use crate::image_processing::{greyscale, gaussian_blur, sobel, non_maximum_suppression, threshold, blob, gabor_filter};

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
//    let mut blurred = gaussian_blur(&mut greyscaled);
//    let (mut sobeled, directions) = sobel(&mut blurred);
//    let mut suppressed = non_maximum_suppression(&mut sobeled, directions);
//    let mut thresheld = threshold(&mut suppressed, 0.15, 0.45);
//    let blobbed = blob(&mut thresheld);
    let gabored = gabor_filter(&greyscaled, 7, 3.0, 0.1);
    gabored.save("result.jpg").expect("Failed to save");
}
