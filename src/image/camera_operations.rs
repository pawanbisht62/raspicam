use crate::image::settings::{CameraSettings, ImageSettings};
use std::io::Error;
use std::process::{Command, Output};

static TRIGGER_CAMERA: &str = "raspistill";
static CAMERA_SHUTTER_SPEED: &str = "-t";
static IMAGE_WIDTH: &str = "-w";
static IMAGE_HEIGHT: &str = "-h";
static CLICKED_IMAGE_PATH: &str = "-o";
static IMAGE_CONTRAST: &str = "-co";
static IMAGE_SHARPNESS: &str = "-sh";
static IMAGE_BRIGHTNESS: &str = "-br";
static IMAGE_SATURATION: &str = "-sa";
static IMAGE_QUALITY: &str = "-q";
static IMAGE_ISO: &str = "-ISO";
static IMAGE_ROTATION: &str = "-rot";
static IMAGE_HORIZONTAL_FLIP: &str = "-hf";
static IMAGE_VERTICAL_FLIP: &str = "-vf";

/// Click image from RaspberryPi's camera and store that image in the user defined path.
///
/// # Arguments
///
/// * `camera_settings` - Structure of camera settings
///
/// * `image_settings` - Structure of image settings
///
/// # Return
///
/// This function retuns the response in Result enum of std::process::Output and std::io::Error.
///
/// # Example
///
/// ```
/// use picam::image::camera_operations::click_image;
/// use picam::image::settings::{CameraSettings, ImageSettings};
///
/// let camera_settings: CameraSettings = CameraSettings::default();
/// let image_settings: ImageSettings = ImageSettings::default();
/// let result = click_image(camera_settings, image_settings);
/// assert!(result.is_err());//because we don't have camera right now!
/// ```
///
pub fn click_image(
    camera_settings: CameraSettings,
    image_settings: ImageSettings,
) -> Result<Output, Error> {
    Command::new(TRIGGER_CAMERA)
        .args(&[
            IMAGE_CONTRAST,
            camera_settings.contrast,
            IMAGE_SHARPNESS,
            camera_settings.sharpness,
            IMAGE_BRIGHTNESS,
            camera_settings.brightness,
            IMAGE_SATURATION,
            camera_settings.saturation,
            IMAGE_QUALITY,
            camera_settings.quality,
            CAMERA_SHUTTER_SPEED,
            camera_settings.timeout,
            IMAGE_ISO,
            camera_settings.iso,
            CLICKED_IMAGE_PATH,
            camera_settings.output,
            IMAGE_WIDTH,
            image_settings.width,
            IMAGE_HEIGHT,
            image_settings.height,
            IMAGE_ROTATION,
            image_settings.rotation,
            IMAGE_HORIZONTAL_FLIP,
            image_settings.horizontal_flip,
            IMAGE_VERTICAL_FLIP,
            image_settings.vertical_flip,
        ])
        .output()
}
