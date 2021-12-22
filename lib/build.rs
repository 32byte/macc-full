extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    // Set cfg flags depending on release channel
    match version_meta().unwrap().channel {
        Channel::Stable => {
            println!("cargo:warning=Running Stable");
        }
        Channel::Nightly => {
            println!("cargo:warning=Running Nightly");
        }
        _ => {
            println!("cargo:warning=Running Beta or Dev");
        }
    }
}
