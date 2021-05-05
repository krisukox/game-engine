use super::angle::Angle;
use super::radians::Radians;
use crate::graph;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug)]
pub struct Player {
    pub angle: Angle,
    pub position: graph::Coordinate,
    number_of_rays: usize,
}

#[cfg_attr(test, automock)]
impl Player {
    pub fn new(angle: Angle, position: graph::Coordinate, number_of_rays: usize) -> Player {
        Player {
            angle,
            position,
            number_of_rays,
        }
    }

    pub fn get_angle_value(&self) -> Radians {
        self.angle.value()
    }

    pub fn get_rays_angle_range(&self) -> std::vec::Vec<std::ops::Range<usize>> {
        self.angle.get_rays_angle_range(self.number_of_rays)
    }

    pub fn get_all_rays(&self) -> Vec<graph::LinearGraph> {
        graph::LinearGraph::get_all_rays(self.number_of_rays)
    }

    pub fn rotate_left(&mut self, angle_delta: Radians) {
        self.angle.rotate_left(angle_delta);
    }

    pub fn rotate_right(&mut self, angle_delta: Radians) {
        self.angle.rotate_right(angle_delta);
    }

    pub fn move_forward_backward(&mut self, distance: f64) {
        let direction = graph::LinearGraph::from_radians(self.angle.get_direction());
        self.position = direction.get_next_from_distance(&self.position, distance);
    }

    pub fn move_right_left(&mut self, distance: f64) {
        let direction =
            graph::LinearGraph::from_radians(self.angle.get_direction() - Radians::PI / 2.0);
        self.position = direction.get_next_from_distance(&self.position, distance);
    }

    pub fn change_position(&mut self, position_delta: &graph::Coordinate) {
        self.position += position_delta;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn player_get_angle_value() {
        let angle_start = 1.3;
        let angle_end = 4.3;
        let player = Player::new(
            Angle {
                start: Radians::new(angle_start),
                end: Radians::new(angle_end),
            },
            graph::Coordinate { x: 0.0, y: 0.0 },
            100,
        );
        assert_eq!(
            player.get_angle_value(),
            Radians::new(angle_end - angle_start)
        );
    }

    #[test]
    fn get_all_rays() {
        let number_of_rays = 100;
        let player = Player::new(
            Angle {
                start: Default::default(),
                end: Default::default(),
            },
            graph::Coordinate { x: 0.0, y: 0.0 },
            number_of_rays,
        );
        assert_eq!(player.get_all_rays().len(), number_of_rays);
    }

    #[test]
    fn player_rotate() {
        let rotate_delta = 1.7;
        let angle_start = 1.3;
        let angle_end = 4.3;
        let mut player = Player::new(
            Angle {
                start: Radians::new(angle_start),
                end: Radians::new(angle_end),
            },
            graph::Coordinate { x: 0.0, y: 0.0 },
            100,
        );
        player.rotate_left(Radians::new(rotate_delta));
        assert_eq!(player.angle.start, Radians::new(angle_start + rotate_delta));
        assert_eq!(player.angle.end, Radians::new(angle_end + rotate_delta));
    }

    #[test]
    fn player_change_position() {
        let change_position_delta = graph::Coordinate { x: 0.2, y: 0.5 };

        let coordinate_x = 1.3;
        let coordinate_y = 4.7;
        let mut player = Player::new(
            Angle {
                start: Default::default(),
                end: Default::default(),
            },
            graph::Coordinate {
                x: coordinate_x,
                y: coordinate_y,
            },
            100,
        );
        assert_eq!(player.position.x, coordinate_x);
        assert_eq!(player.position.y, coordinate_y);

        player.change_position(&change_position_delta);
        assert_eq!(player.position.x, coordinate_x + change_position_delta.x);
        assert_eq!(player.position.y, coordinate_y + change_position_delta.y);
    }

    #[test]
    fn move_player() {
        let angle = Angle {
            start: Radians::ZERO,
            end: Radians::PI / 2.0,
        };
        let first_position = graph::Coordinate { x: 5.0, y: 8.0 };
        let second_position = graph::Coordinate { x: 6.0, y: 9.0 };
        let third_position = graph::Coordinate { x: 6.0, y: 7.0 };
        let distance = 2.0_f64.sqrt();

        let mut player = Player {
            angle,
            position: first_position.clone(),
            number_of_rays: 0,
        };
        player.move_forward_backward(distance);
        assert_eq!(player.position, second_position);

        player.move_forward_backward(-distance);
        assert_eq!(player.position, first_position);

        player.move_right_left(distance);
        assert_eq!(player.position, third_position);

        player.move_right_left(-distance);
        assert_eq!(player.position, first_position);
    }
}
