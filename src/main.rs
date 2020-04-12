use std::env;
mod lib;
use lib::av_bv;

fn main() {
    let param = env::args().nth(1);
    match param {
        Some(param_str) => match param_str.parse::<u64>() {
            Ok(av) => println!("{}", av_bv::atob(&av)),
            _ => println!("{}", av_bv::btoa(&param_str).expect("Invalid BV provided.")),
        },
        None => println!("{}", "Please enter AV / BV."),
    };
}
