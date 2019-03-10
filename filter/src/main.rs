extern crate image;

use crate::image_processing::greyscale;

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

    greyscale(&mut img)
        .save("result.jpg").expect("Failed to save");

}
