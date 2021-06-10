use crate::map_element::{Color, MapElement, Point};

#[cfg(test)]
use crate::graph::Coordinate;

#[derive(Clone)]
pub struct WallMap {
    image: image::RgbaImage,
    color: Color,
}

impl WallMap {
    pub fn new(str_path: &str, color: Option<Color>) -> Result<Self, image::ImageError> {
        let path = std::path::Path::new(str_path);
        let pic = image::open(path)?;
        return Ok(Self {
            image: pic.to_rgba(),
            color: color.unwrap_or(Color::Orange),
        });
    }

    #[cfg(test)]
    fn validate_coordinate(&self, coordinate: &Coordinate) -> bool {
        if coordinate.x < 0.0
            || coordinate.y < 0.0
            || coordinate.x > self.image.width() as f64 - 1.0
            || coordinate.y > self.image.height() as f64 - 1.0
        {
            return false;
        }
        return true;
    }
}

impl MapElement for WallMap {
    fn is_point_in_object(&self, point: &Point) -> bool {
        let pixel = self.image.get_pixel(point.x as u32, point.y as u32);
        return pixel[0] < 100 && pixel[1] < 100 && pixel[2] < 100;
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point_in_object() {
        if let Ok(wall_map) = WallMap::new("test_resources/map-test.png", None) {
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
    fn color() {
        if let Ok(wall_map) = WallMap::new("test_resources/map-test.png", None) {
            assert_eq!(wall_map.color(), Color::Orange);
        } else {
            panic!("File with image for the testcase doesn't exist");
        }
        let color = Color::Green;
        if let Ok(wall_map) = WallMap::new("test_resources/map-test.png", Some(color.clone())) {
            assert_eq!(wall_map.color(), color);
        }
    }

    #[test]
    fn validate_coordinate() {
        if let Ok(wall_map) = WallMap::new("test_resources/map-test.png", None) {
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
        if let Err(image::ImageError::IoError(err)) =
            WallMap::new("dummy_path/dummy_picture.png", None)
        {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("new_map_file_not_found test failed");
        }
    }
}
