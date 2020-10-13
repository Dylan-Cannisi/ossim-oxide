use std::env;

use ossim_oxide::image::Loadable;
use ossim_oxide::tiff::Tiff;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let tiff = Tiff::load(filename.to_string()).unwrap();
    println!("{}", tiff);
}