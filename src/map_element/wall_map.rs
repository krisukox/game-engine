use crate::map_element::{Color, MapElement, Point};
use crate::Map;
use std::path::Path;

#[cfg(test)]
use crate::graph::Coordinate;

#[derive(Clone)]
pub struct WallMap {
    image: image::RgbaImage,
    color: Color,
}

impl WallMap {
    pub fn new(path: &Path, color: Option<Color>) -> Result<Self, image::ImageError> {
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

    pub fn get_map(&self) -> Map {
        Map {
            width: self.image.width() as i64,
            height: self.image.height() as i64,
        }
    }

    pub fn is_black_pixel(&self, x: u32, y: u32) -> bool {
        let pixel = self.image.get_pixel(x, y);
        pixel[0] < 100 && pixel[1] < 100 && pixel[2] < 100
    }
}

impl MapElement for WallMap {
    fn is_point_in_object(&self, point: &Point) -> bool {
        self.is_black_pixel(point.x as u32, point.y as u32)
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::graph::{LinearGraph, MockGraphMethods, Wall};
    use lazy_static::*;

    #[test]
    fn is_point_in_object() {
        if let Ok(wall_map) = WallMap::new(&Path::new("test_resources/map-test.png"), None) {
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
        if let Ok(wall_map) = WallMap::new(&Path::new("test_resources/map-test.png"), None) {
            assert_eq!(wall_map.color(), Color::Orange);
        } else {
            panic!("File with image for the testcase doesn't exist");
        }
        let color = Color::Green;
        if let Ok(wall_map) = WallMap::new(
            &Path::new("test_resources/map-test.png"),
            Some(color.clone()),
        ) {
            assert_eq!(wall_map.color(), color);
        }
    }

    #[test]
    fn validate_coordinate() {
        if let Ok(wall_map) = WallMap::new(&Path::new("test_resources/map-test.png"), None) {
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
            WallMap::new(&Path::new("dummy_path/dummy_picture.png"), None)
        {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("new_map_file_not_found test failed");
        }
    }

    #[test]
    fn get_map() {
        if let Ok(wall_map) = WallMap::new(&Path::new("test_resources/map-test.png"), None) {
            let map = wall_map.get_map();
            assert_eq!(map.width, 50);
            assert_eq!(map.height, 50);
        } else {
            panic!("File with image for the testcase doesn't exist");
        }
    }

    #[test]
    fn is_coordinate_in_object() {
        let color = Color::Green;
        if let Ok(wall_map) = WallMap::new(
            &Path::new("test_resources/map-test.png"),
            Some(color.clone()),
        ) {
            static coordinate_1: Coordinate = Coordinate { x: 34.6, y: 26.0 };
            static coordinate_2: Coordinate = Coordinate { x: 35.4, y: 32.0 };
            static coordinate_3: Coordinate = Coordinate { x: 32.0, y: 29.3 };
            static coordinate_4: Coordinate = Coordinate { x: 36.0, y: 28.7 };
            static coordinate_none: Coordinate = Coordinate { x: 18.0, y: 21.0 };
            static coordinate_out_of_map_1: Coordinate = Coordinate { x: 25.0, y: 0.5 };
            static coordinate_out_of_map_2: Coordinate = Coordinate { x: 0.7, y: 25.0 };

            lazy_static! {
                static ref start_position: Coordinate = Coordinate::default();
                static ref linear_graph: LinearGraph = LinearGraph::default();
            }

            let from_two_coordinates_context = MockGraphMethods::from_two_coordinates_context();

            from_two_coordinates_context
                .expect()
                .once()
                .withf(|first_coordinate, second_coordinate| {
                    *first_coordinate == *start_position
                        && *second_coordinate
                            == Coordinate {
                                x: coordinate_1.x.floor() - 0.0001,
                                y: coordinate_1.y,
                            }
                })
                .return_const(linear_graph.clone());
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_1, &start_position),
                Some((
                    Wall {
                        start_point: Point {
                            x: coordinate_1.x.ceil() as i64,
                            y: coordinate_1.y as i64
                        },
                        end_point: Point {
                            x: coordinate_1.x.floor() as i64,
                            y: coordinate_1.y as i64
                        },
                        primary_object_color: color.clone()
                    },
                    linear_graph.clone()
                ))
            );

            from_two_coordinates_context
                .expect()
                .once()
                .withf(|first_coordinate, second_coordinate| {
                    *first_coordinate == *start_position
                        && *second_coordinate
                            == Coordinate {
                                x: coordinate_2.x.ceil() + 0.0001,
                                y: coordinate_2.y,
                            }
                })
                .return_const(linear_graph.clone());
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_2, &start_position),
                Some((
                    Wall {
                        start_point: Point {
                            x: coordinate_2.x.floor() as i64,
                            y: coordinate_2.y as i64
                        },
                        end_point: Point {
                            x: coordinate_2.x.ceil() as i64,
                            y: coordinate_2.y as i64
                        },
                        primary_object_color: color.clone()
                    },
                    linear_graph.clone()
                ))
            );

            from_two_coordinates_context
                .expect()
                .once()
                .withf(|first_coordinate, second_coordinate| {
                    *first_coordinate == *start_position
                        && *second_coordinate
                            == Coordinate {
                                x: coordinate_3.x,
                                y: coordinate_3.y.ceil() + 0.0001,
                            }
                })
                .return_const(linear_graph.clone());
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_3, &start_position),
                Some((
                    Wall {
                        start_point: Point {
                            x: coordinate_3.x as i64,
                            y: coordinate_3.y.floor() as i64
                        },
                        end_point: Point {
                            x: coordinate_3.x as i64,
                            y: coordinate_3.y.ceil() as i64
                        },
                        primary_object_color: color.clone()
                    },
                    linear_graph.clone()
                ))
            );

            from_two_coordinates_context
                .expect()
                .once()
                .withf(|first_coordinate, second_coordinate| {
                    *first_coordinate == *start_position
                        && *second_coordinate
                            == Coordinate {
                                x: coordinate_4.x,
                                y: coordinate_4.y.floor() - 0.0001,
                            }
                })
                .return_const(linear_graph.clone());
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_4, &start_position),
                Some((
                    Wall {
                        start_point: Point {
                            x: coordinate_4.x as i64,
                            y: coordinate_4.y.ceil() as i64
                        },
                        end_point: Point {
                            x: coordinate_4.x as i64,
                            y: coordinate_4.y.floor() as i64
                        },
                        primary_object_color: color.clone()
                    },
                    linear_graph.clone()
                ))
            );
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_none, &start_position),
                None
            );
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_out_of_map_1, &start_position),
                None
            );
            assert_eq!(
                wall_map.is_coordinate_in_object(&coordinate_out_of_map_2, &start_position),
                None
            );
        } else {
            panic!("File with image for the testcase doesn't exist");
        }
    }
}
