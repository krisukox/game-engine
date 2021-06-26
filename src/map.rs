use crate::graph::{Coordinate, Wall};
use crate::map_element::MapElement;

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

    fn get_wall(
        &self,
        position: &Coordinate, // has to return coordinates sorted in clockwise order
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Option<Wall> {
        for map_element in map_elements {
            let wall = map_element.is_coordinate_in_object(&position);
            if wall != None {
                return wall;
            }
        }
        return None;
    }

    pub(crate) fn cast_ray(
        &self,
        position: &Coordinate,
        ray: &LinearGraph,
        map_elements: &Vec<Box<dyn MapElement>>,
    ) -> Option<Wall> {
        let mut last_position = position.clone();
        let mut next_position: Coordinate;
        loop {
            next_position = ray.get_next(&last_position);
            if !self.validate_coordinate(&next_position) {
                return None;
            }
            let wall = self.get_wall(&next_position, map_elements);
            if wall != None {
                return wall;
            }
            last_position = next_position;
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::graph::MockLinearGraph;
    use crate::map_element::{Color, MockMapElement, Point};
    use mockall::*;

    #[test]
    fn cast_ray_complex() {
        let mut seq = Sequence::new();

        let map = Map {
            width: 50,
            height: 50,
        };

        lazy_static! {
            static ref current_position: Coordinate = Coordinate { x: 30.0, y: 20.0 };
            static ref next_position_1: Coordinate = Coordinate { x: 40.0, y: 30.5 };
            static ref next_position_2: Coordinate = Coordinate { x: 45.0, y: 35.5 };
        }
        let wall = Some(Wall {
            start_point: Point { x: 10, y: 15 },
            end_point: Point { x: 20, y: 25 },
            primary_object_color: Color::Blue,
        });

        let mut ray = MockLinearGraph::new();
        let mut map_element = Box::new(MockMapElement::new());

        ray.expect_get_next()
            .times(1)
            .withf(|coordinate| *coordinate == *current_position)
            .return_const(next_position_1.clone())
            .in_sequence(&mut seq);

        map_element
            .expect_is_coordinate_in_object()
            .times(1)
            .withf(|coordinate| *coordinate == *next_position_1)
            .return_const(None)
            .in_sequence(&mut seq);

        ray.expect_get_next()
            .times(1)
            .withf(|coordinate| *coordinate == *next_position_1)
            .return_const(next_position_2.clone())
            .in_sequence(&mut seq);

        map_element
            .expect_is_coordinate_in_object()
            .times(1)
            .withf(|coordinate| *coordinate == *next_position_2)
            .return_const(wall.clone())
            .in_sequence(&mut seq);

        let ret_wall = map.cast_ray(&current_position, &ray, &vec![map_element]);

        assert_eq!(wall, ret_wall);
    }

    #[test]
    fn cast_ray_out_of_map() {
        let mut seq = Sequence::new();

        let map = Map {
            width: 50,
            height: 50,
        };

        static current_positon: Coordinate = Coordinate { x: 30.0, y: 20.0 };
        let next_position_out_of_map = Coordinate { x: 55.0, y: 30.0 };

        let mut ray = MockLinearGraph::new();
        let map_element = Box::new(MockMapElement::new());

        ray.expect_get_next()
            .times(1)
            .withf(move |coordinate| *coordinate == current_positon)
            .return_const(next_position_out_of_map)
            .in_sequence(&mut seq);

        assert_eq!(
            map.cast_ray(&current_positon, &ray, &vec![map_element]),
            None
        );
    }
}
