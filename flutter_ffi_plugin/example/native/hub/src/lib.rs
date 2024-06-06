mod messages;
mod sample_functions;
// use libvips::ops;
// use libvips::VipsApp;
// use libvips::VipsImage;
// use rinf::debug_print;
use tokio;

rinf::write_interface!();

async fn main() {
    tokio::spawn(sample_functions::calculate_precious_data());
    // tokio::spawn(libvips_add());
}

// pub async fn libvips_add() {
//     use messages::counter_number::*;

//     let mut receiver = MyPreciousData1::get_dart_signal_receiver();
//     debug_print!("=========");

//     MyPreciousData {
//         input_string: 4450.to_string(),
//     }
//     .send_signal_to_dart();

//     while let Some(dart_signal) = receiver.recv().await {
//         let my_precious_data = dart_signal.message;

//         let new_numbers: Vec<i32> = my_precious_data
//             .input_numbers
//             .into_iter()
//             .map(|x| x + 1)
//             .collect();
//         let new_string = my_precious_data.input_string.to_uppercase();

//         debug_print!("{new_numbers:?}");
//         debug_print!("{new_string}");
//     }

//     VipsApp::new("vips_example", true).expect("Failed to initialize libvips");
//     let image = VipsImage::new_from_file("wallpaper.jpg").unwrap();
//     let _resized: VipsImage = ops::resize(&image, 0.5).unwrap();
//     let _options = ops::JpegsaveOptions {
//         q: 90,
//         background: vec![255.0],
//         optimize_coding: true,
//         optimize_scans: true,
//         interlace: true,
//         ..ops::JpegsaveOptions::default()
//     };
//     let _ = ops::jpegsave_with_opts(&_resized, "output.jpg", &_options);
// }
