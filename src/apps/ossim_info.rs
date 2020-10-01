use std::env;

use ossim_oxide::nitf::{Loadable, Nitf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let nitf = Nitf::load(filename.to_string()).unwrap();
    println!("{}",nitf);
}