
pub mod text_cleaner {

    use image::{self, ImageBuffer, Luma};
    use std::io::{Cursor};
    
    const MAX_WIDTH: u32 = 150;
    const MAX_HEIGHT: u32 = 50;

    pub struct Image {
        pub image: image::GrayImage,
    }

    impl Image {
        pub fn new(path: &str) -> Image {
            Self {
                image: image::open(path).unwrap().to_luma8(),
            }
        }

        pub fn binarize(&self) -> Vec<u8> {

            let mut width = self.image.width();
            let mut height = self.image.height();

            if width < MAX_WIDTH && height < MAX_HEIGHT {
                width *= 4;
                height *= 4;
            }
            let mut image;
            image = image::imageops::resize(&self.image, width, height, image::imageops::FilterType::Triangle);
            // Otsu's threshold to obtain binary image
            image = imageproc::contrast::threshold(&image, 130);
            // Gaussian blur and invert image
            image = imageproc::filter::gaussian_blur_f32(&image, 0.25);
            image = imageproc::contrast::adaptive_threshold(&image, 1 as u32);

            self.get_tiff_buffer(&image)
        }

        /// Get the tiff buffer from the image.
        fn get_tiff_buffer(&self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
            let mut buffer = Vec::new();
            image.write_to(
                &mut Cursor::new(&mut buffer),
                image::ImageOutputFormat::Tiff,
            ).unwrap();
            buffer
        }
    }
    
}
