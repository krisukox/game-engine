use crate::graph::Coordinate;
use crate::map_element::MapElement;
use crate::map_element::Point;

#[derive(Clone)]
pub struct WallMap(image::RgbaImage);
/*
const ERROR_STRING_WRONG_FORMAT: &str = "Image should be in format RGBA 8 bit";
*/
impl WallMap {
    pub fn new(str_path: &str) -> Result<Self, image::ImageError> {
        let path = std::path::Path::new(str_path);
        let pic = image::open(path)?;
        return Ok(Self(pic.to_rgba()));
    }

    fn validate_coordinate(&self, coordinate: &Coordinate) -> bool {
        if coordinate.x < 0.0
            || coordinate.y < 0.0
            || coordinate.x > self.0.width() as f64 - 1.0
            || coordinate.y > self.0.height() as f64 - 1.0
        {
            return false;
        }
        return true;
    }
}

impl MapElement for WallMap {
    fn is_point_in_object(&self, point: &Point) -> bool {
        let pixel = self.0.get_pixel(point.x as u32, point.y as u32);
        return pixel[0] < 100 && pixel[1] < 100 && pixel[2] < 100;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player_utils::Radians;

    #[test]
    fn is_point_in_object() {
        if let Ok(wall_map) = WallMap::new("test_resources/map-test.png") {
            assert_eq!(wall_map.is_point_in_object(&Point { x: 23, y: 7 }), true);
            assert_eq!(wall_map.is_point_in_object(&Point { x: 43, y: 23 }), true);
            assert_eq!(wall_map.is_point_in_object(&Point { x: 32, y: 30 }), true);

            assert_eq!(wall_map.is_point_in_object(&Point { x: 18, y: 2 }), false);
            assert_eq!(wall_map.is_point_in_object(&Point { x: 31, y: 16 }), false);
            assert_eq!(wall_map.is_point_in_object(&Point { x: 34, y: 32 }), false);
        } else {
            panic!("File with image for the testcase doesn't exist");
        }
    }

    #[test]
    fn validate_coordinate() {
        if let Ok(wall_map) = WallMap::new("test_resources/map-test.png") {
            assert_eq!(
                wall_map.validate_coordinate(&Coordinate { x: 49.0, y: 49.0 }),
                true
            );
            assert_eq!(
                wall_map.validate_coordinate(&Coordinate { x: 0.0, y: 0.0 }),
                true
            );
            assert_eq!(
                wall_map.validate_coordinate(&Coordinate { x: 39.0, y: 18.0 }),
                true
            );

            assert_eq!(
                wall_map.validate_coordinate(&Coordinate { x: -1.0, y: 20.0 }),
                false
            );
            assert_eq!(
                wall_map.validate_coordinate(&Coordinate { x: 30.0, y: 50.0 }),
                false
            );
            assert_eq!(
                wall_map.validate_coordinate(&Coordinate { x: 59.0, y: 52.0 }),
                false
            );
        } else {
            panic!("File with image for the testcase doesn't exist");
        }
    }

    #[test]
    fn new_map_file_not_found() {
        if let Err(image::ImageError::IoError(err)) = WallMap::new("dummy_path/dummy_picture.png") {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("new_map_file_not_found test failed");
        }
    }
}
