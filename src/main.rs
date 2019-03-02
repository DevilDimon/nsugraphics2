extern crate image;

mod multispectral;

fn main() {
    let img_path = std::env::args().nth(1)
        .expect("no path given");
    let img = image::open(img_path)
        .expect("Cannot find the image at the given path");
}
