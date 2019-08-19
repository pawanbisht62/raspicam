extern crate picam;

use picam::image::camera_operations::click_image;
use picam::image::settings::{CameraSettings, ImageSettings};
use std::io::Error;
use std::process::Output;

pub fn main() {
    // Initialize camera settings with their default values.
    let camera_settings: CameraSettings = CameraSettings::default();

    // Initialize image settings with their default values.
    let image_settings: ImageSettings = ImageSettings::default();

    // Capture image using click_image() function.
    let result: Result<Output, Error> = click_image(camera_settings, image_settings);

    // Print the resultant output.
    println!("{:?}", result);
}
