use crate::graph;

pub struct Map(image::RgbaImage);

const ERROR_STRING_WRONG_FORMAT: &str = "Image should be in format RGBA 8 bit";

impl Map {
    pub fn new(str_path: &str) -> Result<Map, image::ImageError> {
        let path = std::path::Path::new(str_path);
        let pic = image::open(path)?;
        if let image::ImageRgba8(value) = pic {
            return Ok(Map(value));
        }
        return Err(image::ImageError::FormatError(String::from(
            ERROR_STRING_WRONG_FORMAT,
        )));
    }

    fn validate_coordinate(&self, coordinate: &graph::Coordinate) -> bool {
        if coordinate.x < 0.0
            || coordinate.y < 0.0
            || coordinate.x > self.0.width() as f64 - 1.0
            || coordinate.y > self.0.height() as f64 - 1.0
        {
            return false;
        }
        return true;
    }

    fn is_black_pixel(&self, coordinate: &graph::Coordinate) -> bool {
        let pixel = self.0.get_pixel(coordinate.x as u32, coordinate.y as u32);
        return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_black_pixel() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            assert_eq!(
                map.is_black_pixel(&graph::Coordinate { x: 23.0, y: 7.0 }),
                true
            );
            assert_eq!(
                map.is_black_pixel(&graph::Coordinate { x: 43.0, y: 23.0 }),
                true
            );
            assert_eq!(
                map.is_black_pixel(&graph::Coordinate { x: 32.0, y: 30.0 }),
                true
            );

            assert_eq!(
                map.is_black_pixel(&graph::Coordinate { x: 18.0, y: 2.0 }),
                false
            );
            assert_eq!(
                map.is_black_pixel(&graph::Coordinate { x: 31.0, y: 16.0 }),
                false
            );
            assert_eq!(
                map.is_black_pixel(&graph::Coordinate { x: 34.0, y: 32.0 }),
                false
            );
        } else {
            panic!("File with image for the testcase doesn't exist or format is not RGBA 8 bit");
        }
    }

    #[test]
    fn validate_coordinate() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            assert_eq!(
                map.validate_coordinate(&graph::Coordinate { x: 49.0, y: 49.0 }),
                true
            );
            assert_eq!(
                map.validate_coordinate(&graph::Coordinate { x: 0.0, y: 0.0 }),
                true
            );
            assert_eq!(
                map.validate_coordinate(&graph::Coordinate { x: 39.0, y: 18.0 }),
                true
            );

            assert_eq!(
                map.validate_coordinate(&graph::Coordinate { x: -1.0, y: 20.0 }),
                false
            );
            assert_eq!(
                map.validate_coordinate(&graph::Coordinate { x: 30.0, y: 50.0 }),
                false
            );
            assert_eq!(
                map.validate_coordinate(&graph::Coordinate { x: 59.0, y: 52.0 }),
                false
            );
        } else {
            panic!("File with image for the testcase doesn't exist or format is not RGBA 8 bit");
        }
    }

    #[test]
    fn new_map_file_not_found() {
        if let Err(image::ImageError::IoError(err)) = Map::new("dummy_path/dummy_picture.png") {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("new_map_file_not_found test failed");
        }
    }

    #[test]
    fn new_map_wrong_format_image() {
        if let Err(image::ImageError::FormatError(err_str)) =
            Map::new("test_resources/map-wrong-format.jpeg")
        {
            assert_eq!(err_str, ERROR_STRING_WRONG_FORMAT);
        } else {
            panic!("new_map_image_wrong_format test failed");
        }
    }
}
