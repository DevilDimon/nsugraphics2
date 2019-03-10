extern crate image;

mod image_details;

fn main() {
    let filename = std::env::args().nth(1)
        .expect("no path given");
    let img = image::open(std::path::Path::new(&filename))
        .expect("Cannot find the image at the given path");

    println!("{}", image_details::extract_image_details(std::path::Path::new(&filename)
        .file_stem()
        .expect("Wrong filename")
        .to_str()
        .expect("Wrong filename")));
}
