use crate::generator::Polygon;
use crate::graph;
use crate::player_utils;

#[cfg(test)]
use mockall::{automock, predicate::*};

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use super::point_generator::MockPointGenerator as PointGenerator;
    } else {
        use super::point_generator::PointGenerator;
    }
}

pub struct PolygonGenerator {
    pub point_generator: PointGenerator,
}

#[cfg_attr(test, automock)]
impl PolygonGenerator {
    pub fn generate_polygon(
        &self,
        wall: &graph::Wall,
        position: &graph::Coordinate,
        angle: &player_utils::Angle,
    ) -> Polygon {
        let start_point_width =
            self.point_generator
                .point_width(angle, position, &wall.start_point);
        let start_point_height =
            self.point_generator
                .point_height(angle, position, &wall.start_point);
        let end_point_width = self
            .point_generator
            .point_width(angle, position, &wall.end_point);
        let end_point_height = self
            .point_generator
            .point_height(angle, position, &wall.end_point);
        return Polygon {
            area: [
                [start_point_width, start_point_height],
                [end_point_width, end_point_height],
                [end_point_width, -end_point_height],
                [start_point_width, -start_point_height],
            ],
            color: wall.primary_object_color.clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::*;
    use crate::generator::MockPointGenerator;
    use crate::map_element::Color;
    use crate::map_element::Point;
    use mockall::*;

    #[test]
    fn generate_polygon() {
        static angle: player_utils::Angle = player_utils::Angle {
            start: player_utils::Radians::PI,
            end: player_utils::Radians::PI_2,
        };
        static position: graph::Coordinate = graph::Coordinate { x: 11.0, y: 13.0 };
        static color: Color = Color::Yellow;
        static wall: graph::Wall = graph::Wall {
            start_point: Point { x: 1, y: 3 },
            end_point: Point { x: 5, y: 8 },
            primary_object_color: Color::Yellow,
        };

        let start_point_width = 15.0;
        let start_point_height = 17.0;
        let end_point_width = 19.0;
        let end_point_height = 21.0;

        let mut point_generator = MockPointGenerator::default();
        let mut seq = Sequence::new();

        point_generator
            .expect_point_width()
            .times(1)
            .withf(
                |angle_: &player_utils::Angle,
                 start_position: &graph::Coordinate,
                 end_position: &Point| {
                    *angle_ == angle
                        && *start_position == position
                        && *end_position == wall.start_point
                },
            )
            .return_const(start_point_width)
            .in_sequence(&mut seq);

        point_generator
            .expect_point_height()
            .times(1)
            .withf(
                |angle_: &player_utils::Angle,
                 start_position: &graph::Coordinate,
                 end_position: &Point| {
                    *angle_ == angle
                        && *start_position == position
                        && *end_position == wall.start_point
                },
            )
            .return_const(start_point_height)
            .in_sequence(&mut seq);

        point_generator
            .expect_point_width()
            .times(1)
            .withf(
                |angle_: &player_utils::Angle,
                 start_position: &graph::Coordinate,
                 end_position: &Point| {
                    *angle_ == angle
                        && *start_position == position
                        && *end_position == wall.end_point
                },
            )
            .return_const(end_point_width)
            .in_sequence(&mut seq);

        point_generator
            .expect_point_height()
            .times(1)
            .withf(
                |angle_: &player_utils::Angle,
                 start_position: &graph::Coordinate,
                 end_position: &Point| {
                    *angle_ == angle
                        && *start_position == position
                        && *end_position == wall.end_point
                },
            )
            .return_const(end_point_height)
            .in_sequence(&mut seq);

        assert_eq!(
            PolygonGenerator { point_generator }.generate_polygon(&wall, &position, &angle),
            Polygon {
                area: [
                    [start_point_width, start_point_height],
                    [end_point_width, end_point_height],
                    [end_point_width, -end_point_height],
                    [start_point_width, -start_point_height]
                ],
                color: color.clone()
            }
        );
    }
}
