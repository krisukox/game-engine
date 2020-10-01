use super::angle::Angle;
use crate::graph::Coordinate;

#[derive(Debug)]
pub struct Player {
    angle: Angle,
    position: Coordinate,
}

impl Player {
    pub fn new(angle: Angle, position: Coordinate) -> Player {
        Player { angle, position }
    }

    pub fn get_angle_value(&self) -> f64 {
        return self.angle.value();
    }

    pub fn rotate(&mut self, angle_delta: f64) {
        self.angle.rotate(angle_delta);
    }

    pub fn change_position(&mut self, position_delta: &Coordinate) {
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
                start: angle_start,
                end: angle_end,
            },
            Coordinate { x: 0.0, y: 0.0 },
        );
        assert_eq!(player.get_angle_value(), angle_end - angle_start);
    }

    #[test]
    fn player_rotate() {
        let rotate_delta = 1.7;
        let angle_start = 1.3;
        let angle_end = 4.3;
        let mut player = Player::new(
            Angle {
                start: angle_start,
                end: angle_end,
            },
            Coordinate { x: 0.0, y: 0.0 },
        );
        player.rotate(rotate_delta);
        assert_eq!(player.angle.start, angle_start + rotate_delta);
        assert_eq!(player.angle.end, angle_end + rotate_delta);
    }

    #[test]
    fn player_change_position() {
        let change_position_delta = Coordinate { x: 0.2, y: 0.5 };

        let coordinate_x = 1.3;
        let coordinate_y = 4.7;
        let mut player = Player::new(
            Angle {
                start: 0.0,
                end: 0.0,
            },
            Coordinate {
                x: coordinate_x,
                y: coordinate_y,
            },
        );
        assert_eq!(player.position.x, coordinate_x);
        assert_eq!(player.position.y, coordinate_y);

        player.change_position(&change_position_delta);
        assert_eq!(player.position.x, coordinate_x + change_position_delta.x);
        assert_eq!(player.position.y, coordinate_y + change_position_delta.y);
    }
}
