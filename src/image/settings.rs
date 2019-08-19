/// Settings for the camera.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::CameraSettings;
///
/// let camera_settings: CameraSettings = CameraSettings {
///        sharpness: "50",//or any value you want to modify
///        ..Default::default()
///    };
/// let test_camera_settings: CameraSettings = CameraSettings {
///           contrast: "50",
///           sharpness: "50",
///           brightness: "60",
///           saturation: "0",
///           quality: "100",
///           timeout: "3000",
///           iso: "300",
///           output: "~/raspicam.jpg",
/// };
///
/// assert_eq!(camera_settings, test_camera_settings);
/// ```
///
#[derive(Debug, PartialEq)]
pub struct CameraSettings {
    pub contrast: &'static str,
    pub sharpness: &'static str,
    pub brightness: &'static str,
    pub saturation: &'static str,
    pub quality: &'static str,
    pub timeout: &'static str,
    pub iso: &'static str,
    pub output: &'static str,
}

impl Default for CameraSettings {
    /// Initialize CameraSettings with the default values
    ///
    /// # Example
    ///
    /// ```
    /// use raspicam::image::settings::CameraSettings;
    ///
    /// let camera_settings: CameraSettings = CameraSettings::default();
    /// let test_camera_settings: CameraSettings = CameraSettings {
    ///           contrast: "50",
    ///           sharpness: "30",
    ///           brightness: "60",
    ///           saturation: "0",
    ///           quality: "100",
    ///           timeout: "3000",
    ///           iso: "300",
    ///           output: "~/raspicam.jpg",
    /// };
    ///
    /// assert_eq!(camera_settings, test_camera_settings);
    /// ```
    ///
    fn default() -> CameraSettings {
        CameraSettings {
            contrast: "50",
            sharpness: "30",
            brightness: "60",
            saturation: "0",
            quality: "100",
            timeout: "3000",
            iso: "300",
            output: "~/raspicam.jpg",
        }
    }
}

/// Settings for the image.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::ImageSettings;
///
/// let image_settings: ImageSettings = ImageSettings {
///        width: "50",//or any value you want to modify
///        ..Default::default()
///    };
/// let test_image_settings: ImageSettings = ImageSettings {
///         width: "50",
///         height: "200",
///         rotation: "180",
///         horizontal_flip: "false",
///         vertical_flip: "false",
/// };
///
/// assert_eq!(image_settings, test_image_settings);
/// ```
///
#[derive(Debug, PartialEq)]
pub struct ImageSettings {
    pub width: &'static str,
    pub height: &'static str,
    pub rotation: &'static str,
    pub horizontal_flip: &'static str,
    pub vertical_flip: &'static str,
}

impl Default for ImageSettings {
    /// Initialize ImageSettings with the default values
    ///
    /// # Example
    ///
    /// ```
    /// use raspicam::image::settings::ImageSettings;
    ///
    /// let image_settings: ImageSettings = ImageSettings::default();
    /// let test_image_settings: ImageSettings = ImageSettings {
    ///         width: "200",
    ///         height: "200",
    ///         rotation: "180",
    ///         horizontal_flip: "false",
    ///         vertical_flip: "false",
    /// };
    ///
    /// assert_eq!(image_settings, test_image_settings);
    /// ```
    ///
    fn default() -> ImageSettings {
        ImageSettings {
            width: "200",
            height: "200",
            rotation: "180",
            horizontal_flip: "false",
            vertical_flip: "false",
        }
    }
}
