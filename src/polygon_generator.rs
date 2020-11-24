use crate::graph;
use crate::player_utils;
use crate::point_generator;
use graphics::types::Vec2d;

#[cfg(test)]
use mockall::{automock, predicate::*};

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use self::point_generator::MockPointGenerator as PointGenerator;
    } else {
        use self::point_generator::PointGenerator;
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
    ) -> [Vec2d; 4] {
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
        return [
            [start_point_width, start_point_height],
            [end_point_width, end_point_height],
            [end_point_width, -end_point_height],
            [start_point_width, -start_point_height],
        ];
    }
}

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]
    use super::*;
    use mockall::*;

    #[test]
    fn generate_polygon() {
        static angle: player_utils::Angle = player_utils::Angle {
            start: player_utils::Radians(0.5),
            end: player_utils::Radians(0.5),
        };
        static position: graph::Coordinate = graph::Coordinate { x: 11.0, y: 13.0 };
        static wall: graph::Wall = graph::Wall {
            start_point: graph::Coordinate { x: 1.0, y: 3.0 },
            end_point: graph::Coordinate { x: 5.0, y: 8.0 },
        };

        let start_point_width = 15.0;
        let start_point_height = 17.0;
        let end_point_width = 19.0;
        let end_point_height = 21.0;

        let mut point_generator = point_generator::MockPointGenerator::default();
        let mut seq = Sequence::new();

        point_generator
            .expect_point_width()
            .times(1)
            .withf(
                |angle_: &player_utils::Angle,
                 start_position: &graph::Coordinate,
                 end_position: &graph::Coordinate| {
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
                 end_position: &graph::Coordinate| {
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
                 end_position: &graph::Coordinate| {
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
                 end_position: &graph::Coordinate| {
                    *angle_ == angle
                        && *start_position == position
                        && *end_position == wall.end_point
                },
            )
            .return_const(end_point_height)
            .in_sequence(&mut seq);

        assert_eq!(
            PolygonGenerator { point_generator }.generate_polygon(&wall, &position, &angle),
            [
                [start_point_width, start_point_height],
                [end_point_width, end_point_height],
                [end_point_width, -end_point_height],
                [start_point_width, -start_point_height]
            ]
        );
    }
}
