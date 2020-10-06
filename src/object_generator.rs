use crate::graph;
use crate::map::Map;

pub struct ObjectGenerator {
    map: Map,
    rays: Vec<graph::LinearGraph>,
}

impl ObjectGenerator {
    pub fn get_points_in_sight(
        &self,
        position: &graph::Coordinate,
        rays_indexes: std::ops::Range<usize>,
    ) -> Vec<graph::Coordinate> {
        let mut points_in_sight: Vec<graph::Coordinate> = Vec::new();
        for index in rays_indexes {
            let points = self.map.cast_ray(position, &self.rays[index]);
            for point in points {
                if !points_in_sight.contains(&point) {
                    points_in_sight.push(point);
                }
            }
        }
        return points_in_sight;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_points_in_sight() {
        let expected_points_in_sight = vec![
            graph::Coordinate { x: 34.0, y: 26.0 },
            graph::Coordinate { x: 34.0, y: 27.0 },
            graph::Coordinate { x: 34.0, y: 28.0 },
            graph::Coordinate { x: 33.0, y: 28.0 },
            graph::Coordinate { x: 32.0, y: 28.0 },
            graph::Coordinate { x: 32.0, y: 29.0 },
            graph::Coordinate { x: 32.0, y: 30.0 },
            graph::Coordinate { x: 31.0, y: 30.0 },
            graph::Coordinate { x: 30.0, y: 30.0 },
            graph::Coordinate { x: 30.0, y: 31.0 },
            graph::Coordinate { x: 30.0, y: 32.0 },
            graph::Coordinate { x: 29.0, y: 32.0 },
            graph::Coordinate { x: 28.0, y: 32.0 },
            graph::Coordinate { x: 27.0, y: 32.0 },
        ];

        let position = graph::Coordinate { x: 27.0, y: 26.0 };

        let mut rays: Vec<graph::LinearGraph> = Vec::new();
        let mut radians = 0.0;
        while radians < std::f64::consts::PI * 2.0 {
            rays.push(graph::LinearGraph::from_radians(radians));
            radians += 0.1;
        }
        let rays_indexes = 0..rays.len() / 4 + 1;
        if let Ok(map) = Map::new("test_resources/map.png") {
            let object_generator = ObjectGenerator { map, rays };
            let points_in_sight = object_generator.get_points_in_sight(&position, rays_indexes);
            assert_eq!(expected_points_in_sight.len(), points_in_sight.len());
            for expected_point in expected_points_in_sight {
                assert!(points_in_sight.contains(&expected_point));
            }
        }
    }
}
