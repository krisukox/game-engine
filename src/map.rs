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

    fn is_black_wall_or_point(
        &self,
        points: Vec<graph::Coordinate>,
    ) -> Option<Vec<graph::Coordinate>> {
        for point in &points {
            if !self.is_black_pixel(&point) {
                return None;
            }
        }
        return Some(points);
    }

    fn get_point_or_wall(
        &self,
        last_position: &graph::Coordinate, // last cordinate is needed because get_point_or_wall
        next_position: &graph::Coordinate, // has to return coordinates sorted in clockwise order
    ) -> Option<Vec<graph::Coordinate>> {
        return self.is_black_wall_or_point(next_position.get_nearest_coordinates(&last_position));
    }

    pub fn cast_ray(
        &self,
        position: &graph::Coordinate,
        ray: &graph::LinearGraph,
    ) -> Vec<graph::Coordinate> {
        let mut last_position = position.clone();
        let mut next_position: graph::Coordinate;
        loop {
            next_position = ray.get_next(&last_position);
            if let Some(points) = self.get_point_or_wall(&last_position, &next_position) {
                return points;
            }
            last_position = next_position;
        }
    }

    pub fn dummy() -> Map {
        Map(image::ImageBuffer::from_pixel(
            1,
            1,
            image::Rgba([255 as u8, 255, 255, 255]),
        ))
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

    #[test]
    fn cast_ray_1() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            let expected_black_points = vec![
                graph::Coordinate { x: 32.0, y: 28.0 },
                graph::Coordinate { x: 32.0, y: 29.0 },
                graph::Coordinate { x: 32.0, y: 30.0 },
                graph::Coordinate { x: 31.0, y: 30.0 },
                graph::Coordinate { x: 30.0, y: 30.0 },
            ];

            let start_position = graph::Coordinate { x: 31.0, y: 29.0 };
            let rays = vec![
                graph::LinearGraph::from_radians(std::f64::consts::PI * 2.0 - 0.5),
                graph::LinearGraph::from_radians(std::f64::consts::PI / 4.0),
                graph::LinearGraph::from_radians(std::f64::consts::PI / 2.0 + 0.5),
            ];
            let mut black_points: Vec<graph::Coordinate> = Vec::new();

            for ray in rays {
                black_points.append(&mut map.cast_ray(&start_position, &ray))
            }

            assert_eq!(expected_black_points.len(), black_points.len());
            for expected_black_point in expected_black_points {
                assert!(black_points.contains(&expected_black_point));
            }
        }
    }

    #[test]
    fn cast_ray_2() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            let start_position = graph::Coordinate { x: 31.0, y: 29.0 };

            let expected_black_points_1 = vec![
                graph::Coordinate { x: 32.0, y: 29.0 },
                graph::Coordinate { x: 32.0, y: 30.0 },
                graph::Coordinate { x: 31.0, y: 30.0 },
            ];
            let rays_1 = vec![
                graph::LinearGraph::from_radians(std::f64::consts::PI / 4.0 + 0.01),
                graph::LinearGraph::from_radians(std::f64::consts::PI / 4.0 - 0.01),
            ];

            let expected_black_point_2 = graph::Coordinate { x: 32.0, y: 30.0 };
            let ray_2 = graph::LinearGraph::from_radians(std::f64::consts::PI / 4.0);

            let mut black_points_1: Vec<graph::Coordinate> = Vec::new();
            for ray in rays_1 {
                black_points_1.append(&mut map.cast_ray(&start_position, &ray))
            }
            assert_eq!(black_points_1.len(), 4);
            for expected_black_point in expected_black_points_1 {
                assert!(black_points_1.contains(&expected_black_point));
            }

            let black_points_2 = map.cast_ray(&start_position, &ray_2);
            assert_eq!(black_points_2.len(), 1);
            assert_eq!(black_points_2[0], expected_black_point_2);
        }
    }

    #[test]
    fn get_point_or_wall_black() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            let dummy_last_position = Default::default();

            let position_black_1 = graph::Coordinate { x: 33.0, y: 30.0 };
            let position_black_2 = graph::Coordinate { x: 33.5, y: 30.5 };

            let black_point_1 = map.get_point_or_wall(&dummy_last_position, &position_black_1);
            let black_point_2 = map.get_point_or_wall(&dummy_last_position, &position_black_2);

            if let Some(black_point_or_wall) = black_point_1 {
                assert_eq!(black_point_or_wall, vec![position_black_1]);
            } else {
                panic!("black_point_or_wall_1 contains None");
            }

            if let Some(black_point_or_wall) = black_point_2 {
                assert_eq!(
                    black_point_or_wall,
                    vec![graph::Coordinate {
                        x: position_black_2.x.round(),
                        y: position_black_2.y.round()
                    }]
                );
            } else {
                panic!("black_point_or_wall_4 contains None");
            }
        }
    }

    #[test]
    fn get_wall_black_check_order() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            let position_black_1 = graph::Coordinate { x: 33.0, y: 30.5 };
            let position_black_2 = graph::Coordinate { x: 33.5, y: 30.0 };

            let last_position_1_larger_x = graph::Coordinate {
                x: position_black_1.x + 1.0,
                y: 25.0,
            };
            let last_position_1_smaller_x = graph::Coordinate {
                x: position_black_1.x - 1.0,
                y: 25.0,
            };
            let last_position_2_larger_y = graph::Coordinate {
                x: 30.0,
                y: position_black_2.y + 1.0,
            };
            let last_position_2_smaller_y = graph::Coordinate {
                x: 30.0,
                y: position_black_2.y - 1.0,
            };

            if let Some(black_wall) =
                map.get_point_or_wall(&last_position_1_larger_x, &position_black_1)
            {
                assert_eq!(
                    black_wall,
                    vec![
                        graph::Coordinate {
                            x: position_black_1.x,
                            y: position_black_1.y.ceil()
                        },
                        graph::Coordinate {
                            x: position_black_1.x,
                            y: position_black_1.y.floor()
                        }
                    ]
                );
            } else {
                panic!("black_point_or_wall_2 contains None");
            }

            if let Some(black_wall) =
                map.get_point_or_wall(&last_position_1_smaller_x, &position_black_1)
            {
                assert_eq!(
                    black_wall,
                    vec![
                        graph::Coordinate {
                            x: position_black_1.x,
                            y: position_black_1.y.floor()
                        },
                        graph::Coordinate {
                            x: position_black_1.x,
                            y: position_black_1.y.ceil()
                        },
                    ]
                );
            } else {
                panic!("black_point_or_wall_2 contains None");
            }

            if let Some(black_wall) =
                map.get_point_or_wall(&last_position_2_larger_y, &position_black_2)
            {
                assert_eq!(
                    black_wall,
                    vec![
                        graph::Coordinate {
                            x: position_black_2.x.floor(),
                            y: position_black_2.y
                        },
                        graph::Coordinate {
                            x: position_black_2.x.ceil(),
                            y: position_black_2.y
                        }
                    ]
                );
            } else {
                panic!("black_point_or_wall_2 contains None");
            }

            if let Some(black_wall) =
                map.get_point_or_wall(&last_position_2_smaller_y, &position_black_2)
            {
                assert_eq!(
                    black_wall,
                    vec![
                        graph::Coordinate {
                            x: position_black_2.x.ceil(),
                            y: position_black_2.y
                        },
                        graph::Coordinate {
                            x: position_black_2.x.floor(),
                            y: position_black_2.y
                        },
                    ]
                );
            } else {
                panic!("black_point_or_wall_2 contains None");
            }
        }
    }

    #[test]
    fn get_point_or_wall() {
        if let Ok(map) = Map::new("test_resources/map.png") {
            let dummy_position = Default::default();

            let position_white_1 = graph::Coordinate { x: 24.0, y: 19.0 };
            let position_white_2 = graph::Coordinate { x: 24.0, y: 19.5 };
            let position_white_3 = graph::Coordinate { x: 24.5, y: 19.0 };
            let position_white_4 = graph::Coordinate { x: 24.5, y: 19.5 };

            assert!(map
                .get_point_or_wall(&position_white_1, &dummy_position)
                .is_none());
            assert!(map
                .get_point_or_wall(&position_white_2, &dummy_position)
                .is_none());
            assert!(map
                .get_point_or_wall(&position_white_3, &dummy_position)
                .is_none());
            assert!(map
                .get_point_or_wall(&position_white_4, &dummy_position)
                .is_none());
        }
    }
}
