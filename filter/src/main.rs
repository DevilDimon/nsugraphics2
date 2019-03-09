extern crate image;

mod image_details;

fn main() {
    let img_path = std::env::args().nth(1)
        .expect("no path given");
    let img = image::open(img_path.clone())
        .expect("Cannot find the image at the given path");

    println!("Image info: {}", image_details::extract_image_details(&img_path));
}
