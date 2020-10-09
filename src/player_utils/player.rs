use super::angle::Angle;
use super::radians::Radians;
use crate::graph;

#[derive(Debug)]
pub struct Player {
    angle: Angle,
    position: graph::Coordinate,
    number_of_rays: usize,
}

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

    pub fn get_all_rays(&self) -> Vec<graph::LinearGraph> {
        graph::LinearGraph::get_all_rays(self.number_of_rays)
    }

    pub fn rotate(&mut self, angle_delta: Radians) {
        self.angle.rotate(angle_delta);
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
                start: Radians(angle_start),
                end: Radians(angle_end),
            },
            graph::Coordinate { x: 0.0, y: 0.0 },
            100,
        );
        assert_eq!(player.get_angle_value(), Radians(angle_end - angle_start));
    }

    #[test]
    fn get_all_rays() {
        let number_of_rays = 100;
        let player = Player::new(
            Angle {
                start: Radians(0.0),
                end: Radians(0.0),
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
                start: Radians(angle_start),
                end: Radians(angle_end),
            },
            graph::Coordinate { x: 0.0, y: 0.0 },
            100,
        );
        player.rotate(Radians(rotate_delta));
        assert_eq!(player.angle.start, Radians(angle_start + rotate_delta));
        assert_eq!(player.angle.end, Radians(angle_end + rotate_delta));
    }

    #[test]
    fn player_change_position() {
        let change_position_delta = graph::Coordinate { x: 0.2, y: 0.5 };

        let coordinate_x = 1.3;
        let coordinate_y = 4.7;
        let mut player = Player::new(
            Angle {
                start: Radians(0.0),
                end: Radians(0.0),
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
}
