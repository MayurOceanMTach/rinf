#![allow(unused_imports)]

mod messages;

use libvips::ops;
use libvips::VipsApp;
use libvips::VipsImage;
use tokio;

rinf::write_interface!();

async fn main() {
    tokio::spawn(libvips_add());
}

pub async fn libvips_add() {
    VipsApp::new("vips_example", true).expect("Failed to initialize libvips");
    let image = VipsImage::new_from_file("wallpaper.jpg").unwrap();
    let _resized: VipsImage = ops::resize(&image, 0.5).unwrap();
    let _options = ops::JpegsaveOptions {
        q: 90,
        background: vec![255.0],
        optimize_coding: true,
        optimize_scans: true,
        interlace: true,
        ..ops::JpegsaveOptions::default()
    };
    match ops::jpegsave_with_opts(&_resized, "output.jpg", &_options) {
        Err(_) => println!("error: {}", 10),
        Ok(_) => println!("Great Success!"),
    }
}
