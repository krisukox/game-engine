use super::angle::Angle;
use super::radians::Radians;
use crate::graph;

#[cfg(test)]
use mockall::{automock, predicate::*};

cfg_if::cfg_if! {
    if #[cfg(test)]{
        use super::move_handler::MockMoveHandler as MoveHandler;
    } else {
        use super::move_handler::MoveHandler;
    }
}
#[cfg_attr(not(test), derive(Debug))]
pub struct Player {
    pub angle: Angle,
    pub position: graph::Coordinate,
    number_of_rays: usize,
    move_handler: MoveHandler,
}

#[cfg_attr(test, automock)]
impl Player {
    pub fn new(angle: Angle, position: graph::Coordinate, number_of_rays: usize) -> Player {
        Player {
            angle,
            position,
            number_of_rays,
            #[cfg(not(test))]
            move_handler: MoveHandler::new(),
            #[cfg(test)]
            move_handler: MoveHandler::default(),
        }
    }

    fn move_forward_backward(&mut self, distance: f64) {
        let direction = graph::LinearGraph::from_radians(self.angle.get_direction());
        self.position = direction.get_next_from_distance(&self.position, distance);
    }

    fn move_right_left(&mut self, distance: f64) {
        let direction =
            graph::LinearGraph::from_radians(self.angle.get_direction() - Radians::PI / 2.0);
        self.position = direction.get_next_from_distance(&self.position, distance);
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

    pub fn move_right(&mut self, is_move: bool) {
        self.move_handler.move_right(is_move)
    }

    pub fn move_left(&mut self, is_move: bool) {
        self.move_handler.move_left(is_move)
    }

    pub fn move_forward(&mut self, is_move: bool) {
        self.move_handler.move_forward(is_move)
    }

    pub fn move_backward(&mut self, is_move: bool) {
        self.move_handler.move_backward(is_move)
    }

    pub fn update(&mut self) {
        if let Some(forward_backward_value) = self.move_handler.get_move_forward_backward_value() {
            self.move_forward_backward(forward_backward_value)
        }

        if let Some(right_left_value) = self.move_handler.get_move_right_left_value() {
            self.move_right_left(right_left_value)
        }
    }

    pub fn change_position(&mut self, position_delta: &graph::Coordinate) {
        self.position += position_delta;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use float_cmp::approx_eq;

    fn check_update(
        forward_backward_value: Option<f64>,
        right_left_value: Option<f64>,
        start_position: graph::Coordinate,
        updated_postion: graph::Coordinate,
    ) {
        let angle = Angle {
            start: Radians::PI / 4.0,
            end: Radians::new(std::f64::consts::PI * 3.0 / 4.0),
        };
        let mut move_handler = MoveHandler::default();

        move_handler
            .expect_get_move_forward_backward_value()
            .times(1)
            .return_const(forward_backward_value);
        move_handler
            .expect_get_move_right_left_value()
            .times(1)
            .return_const(right_left_value);

        let mut player = Player {
            angle,
            position: start_position,
            number_of_rays: 0,
            move_handler,
        };
        player.update();

        assert_eq!(player.position, updated_postion);
    }

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
            move_handler: MoveHandler::default(),
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

    #[test]
    fn move_player_2() {
        let angle = Angle {
            start: Radians::ZERO,
            end: Radians::PI / 2.0,
        };
        let position = graph::Coordinate { x: 5.0, y: 8.0 };
        let mut move_handler = MoveHandler::default();

        move_handler
            .expect_move_right()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == true);
        move_handler
            .expect_move_right()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == false);
        move_handler
            .expect_move_left()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == true);
        move_handler
            .expect_move_left()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == false);
        move_handler
            .expect_move_forward()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == true);
        move_handler
            .expect_move_forward()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == false);
        move_handler
            .expect_move_backward()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == true);
        move_handler
            .expect_move_backward()
            .times(1)
            .return_const(())
            .withf(|is_move| *is_move == false);

        let mut player = Player {
            angle,
            position: position.clone(),
            number_of_rays: 0,
            move_handler,
        };

        player.move_right(true);
        player.move_right(false);
        player.move_left(true);
        player.move_left(false);
        player.move_forward(true);
        player.move_forward(false);
        player.move_backward(true);
        player.move_backward(false);
    }

    #[test]
    fn update_postion() {
        check_update(
            Some(1.0),
            None,
            graph::Coordinate { x: 5.0, y: 8.0 },
            graph::Coordinate { x: 5.0, y: 9.0 },
        );
        check_update(
            Some(-1.0),
            None,
            graph::Coordinate { x: 5.0, y: 8.0 },
            graph::Coordinate { x: 5.0, y: 7.0 },
        );
        check_update(
            None,
            Some(1.0),
            graph::Coordinate { x: 5.0, y: 8.0 },
            graph::Coordinate { x: 6.0, y: 8.0 },
        );
        check_update(
            None,
            Some(-1.0),
            graph::Coordinate { x: 5.0, y: 8.0 },
            graph::Coordinate { x: 4.0, y: 8.0 },
        );
    }
}
