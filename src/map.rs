use crate::graph::Coordinate;
use crate::map_element::{Color, ColoredPoint, MapElement, Point};

#[cfg(test)]
use mockall::automock;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::graph::MockLinearGraph as LinearGraph;
    } else {
        use crate::graph::LinearGraph;
    }
}

#[derive(Default, Clone)]
pub struct Map {
    pub width: i64,
    pub height: i64,
}

#[cfg_attr(test, automock)]
impl Map {
    fn validate_coordinate(&self, coordinate: &Coordinate) -> bool {
        if coordinate.x < 0.0
            || coordinate.y < 0.0
            || coordinate.x > self.width as f64 - 1.0
            || coordinate.y > self.height as f64 - 1.0
        {
            return false;
        }
        return true;
    }

    fn is_black_wall_or_point(
        &self,
        points: Vec<Point>,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Option<Vec<ColoredPoint>> {
        let mut points_primary_object_type = vec![];
        let points_len = points.len();
        'point_loop: for point in points.into_iter() {
            for map_element in map_elements {
                if map_element.is_point_in_object(&point) {
                    points_primary_object_type.push(ColoredPoint {
                        point,
                        color: map_element.color(),
                    });
                    continue 'point_loop;
                }
            }
            return None;
        }

        if points_primary_object_type.len() == points_len {
            return Some(points_primary_object_type);
        }
        return None;
    }

    fn get_point_or_wall(
        &self,
        last_position: &Coordinate, // last cordinate is needed because get_point_or_wall
        next_position: &Coordinate, // has to return coordinates sorted in clockwise order
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Option<Vec<ColoredPoint>> {
        return self.is_black_wall_or_point(
            next_position.get_nearest_points(&last_position),
            map_elements,
        );
    }

    pub(crate) fn cast_ray(
        &self,
        position: &Coordinate,
        ray: &LinearGraph,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Vec<ColoredPoint> {
        let mut last_position = position.clone();
        let mut next_position: Coordinate;
        loop {
            next_position = ray.get_next(&last_position);
            if !self.validate_coordinate(&next_position) {
                return vec![];
            }
            if let Some(points) =
                self.get_point_or_wall(&last_position, &next_position, map_elements)
            {
                return points;
            }
            last_position = next_position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MockLinearGraph;
    use crate::map_element::MockMapElement;
    use mockall::*;

    #[test]
    fn cast_ray_simple() {
        let mut seq = Sequence::new();

        let map = Map {
            width: 50,
            height: 50,
        };

        let current_positon = Coordinate { x: 30.0, y: 20.0 };
        let next_position = Coordinate { x: 40.0, y: 30.0 };
        let next_point = ColoredPoint {
            point: Point {
                x: next_position.x as i64,
                y: next_position.y as i64,
            },
            color: Color::Green,
        };

        let mut ray = MockLinearGraph::new();
        let mut map_element = Box::new(MockMapElement::new());

        let clone_current_position = current_positon.clone();
        ray.expect_get_next()
            .times(1)
            .withf(move |coordinate| *coordinate == clone_current_position)
            .return_const(next_position)
            .in_sequence(&mut seq);

        let clone_next_point = next_point.point.clone();
        map_element
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point| *point == clone_next_point)
            .return_const(true)
            .in_sequence(&mut seq);
        let clone_next_color = next_point.color.clone();
        map_element
            .expect_color()
            .times(1)
            .return_const(clone_next_color)
            .in_sequence(&mut seq);

        assert_eq!(
            map.cast_ray(&current_positon, &ray, &vec![map_element]),
            vec![next_point]
        );
    }

    #[test]
    fn cast_ray_complex() {
        let mut seq = Sequence::new();

        let map = Map {
            width: 50,
            height: 50,
        };

        let current_positon = Coordinate { x: 30.0, y: 20.0 };
        let next_position_1 = Coordinate { x: 40.0, y: 30.5 };
        let next_points_1 = vec![
            Point {
                x: next_position_1.x as i64,
                y: next_position_1.y.ceil() as i64,
            },
            Point {
                x: next_position_1.x as i64,
                y: next_position_1.y.floor() as i64,
            },
        ];

        let next_position_2 = Coordinate { x: 45.0, y: 35.5 };
        let next_points_2 = vec![
            ColoredPoint {
                point: Point {
                    x: next_position_2.x as i64,
                    y: next_position_2.y.ceil() as i64,
                },
                color: Color::Red,
            },
            ColoredPoint {
                point: Point {
                    x: next_position_2.x as i64,
                    y: next_position_2.y.floor() as i64,
                },
                color: Color::Green,
            },
        ];

        let mut ray = MockLinearGraph::new();
        let mut map_element = Box::new(MockMapElement::new());

        let clone_current_position = current_positon.clone();
        ray.expect_get_next()
            .times(1)
            .withf(move |coordinate| *coordinate == clone_current_position)
            .return_const(next_position_1.clone())
            .in_sequence(&mut seq);

        let clone_next_points_1 = next_points_1.clone();
        map_element
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point| clone_next_points_1.contains(point))
            .return_const(false)
            .in_sequence(&mut seq);

        ray.expect_get_next()
            .times(1)
            .withf(move |coordinate| *coordinate == next_position_1)
            .return_const(next_position_2)
            .in_sequence(&mut seq);

        let clone_next_points_2 = next_points_2.clone();
        map_element
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point| {
                clone_next_points_2
                    .iter()
                    .any(|point_| point_.point == *point)
            })
            .return_const(true)
            .in_sequence(&mut seq);
        let clone_next_points_2 = next_points_2.clone();
        map_element
            .expect_color()
            .times(1)
            .return_const(clone_next_points_2[0].color.clone())
            .in_sequence(&mut seq);
        let clone_next_points_2 = next_points_2.clone();
        map_element
            .expect_is_point_in_object()
            .times(1)
            .withf(move |point| {
                clone_next_points_2
                    .iter()
                    .any(|point_| point_.point == *point)
            })
            .return_const(true)
            .in_sequence(&mut seq);
        let clone_next_points_2 = next_points_2.clone();
        map_element
            .expect_color()
            .times(1)
            .return_const(clone_next_points_2[1].color.clone())
            .in_sequence(&mut seq);

        let ret_points = map.cast_ray(&current_positon, &ray, &vec![map_element]);

        for point in &next_points_2 {
            assert!(ret_points
                .iter()
                .any(|ret_point| ret_point.point == point.point));
        }

        for (ret_point, point) in ret_points.iter().zip(next_points_2) {
            assert!(ret_point.color == point.color);
        }
    }

    #[test]
    fn cast_ray_out_of_map() {
        let mut seq = Sequence::new();

        let map = Map {
            width: 50,
            height: 50,
        };

        let current_positon = Coordinate { x: 30.0, y: 20.0 };
        let next_position_out_of_map = Coordinate { x: 55.0, y: 30.0 };

        let mut ray = MockLinearGraph::new();
        let map_element = Box::new(MockMapElement::new());

        let clone_current_position = current_positon.clone();
        ray.expect_get_next()
            .times(1)
            .withf(move |coordinate| *coordinate == clone_current_position)
            .return_const(next_position_out_of_map)
            .in_sequence(&mut seq);

        assert_eq!(
            map.cast_ray(&current_positon, &ray, &vec![map_element]),
            vec![]
        );
    }
}
