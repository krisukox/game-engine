use crate::graph;
use std::time::SystemTime;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug)]
pub struct MoveHandler {
    move_left: bool,
    move_right: bool,
    move_forward: bool,
    move_backward: bool,
    move_left_last_time: SystemTime,
    move_right_last_time: SystemTime,
    move_forward_last_time: SystemTime,
    move_backward_last_time: SystemTime,
    position: graph::Coordinate,
}

#[cfg_attr(test, automock)]
impl MoveHandler {
    pub fn new() -> Self {
        Self {
            move_left: false,
            move_right: false,
            move_forward: false,
            move_backward: false,
            move_left_last_time: SystemTime::now(),
            move_right_last_time: SystemTime::now(),
            move_forward_last_time: SystemTime::now(),
            move_backward_last_time: SystemTime::now(),
            position: graph::Coordinate { x: 0.0, y: 0.0 },
        }
    }

    fn _move(is_move: bool, move_flag: &mut bool, move_last_time: &mut SystemTime) {
        if is_move == true {
            if *move_flag == false {
                *move_last_time = SystemTime::now();
            }
            *move_flag = true;
        } else {
            *move_flag = false;
        }
    }

    pub fn move_left(&mut self, is_move: bool) {
        Self::_move(is_move, &mut self.move_left, &mut self.move_left_last_time);
    }

    pub fn move_right(&mut self, is_move: bool) {
        Self::_move(
            is_move,
            &mut self.move_right,
            &mut self.move_right_last_time,
        );
    }

    pub fn move_forward(&mut self, is_move: bool) {
        Self::_move(
            is_move,
            &mut self.move_forward,
            &mut self.move_forward_last_time,
        );
    }

    pub fn move_backward(&mut self, is_move: bool) {
        Self::_move(
            is_move,
            &mut self.move_backward,
            &mut self.move_backward_last_time,
        );
    }

    pub fn get_move_forward_backward_value(&mut self) -> Option<f64> {
        if self.move_forward == true && self.move_backward == true {
            self.move_forward_last_time = SystemTime::now();
            self.move_backward_last_time = SystemTime::now();
            return None;
        }
        if self.move_forward == true && self.move_backward == false {
            let move_distance =
                self.move_forward_last_time.elapsed().unwrap().as_nanos() as f64 / 50000000.0;
            self.move_forward_last_time = SystemTime::now();
            return Some(move_distance);
        }
        if self.move_backward == true && self.move_forward == false {
            let move_distance =
                -(self.move_backward_last_time.elapsed().unwrap().as_nanos() as f64 / 50000000.0);
            self.move_backward_last_time = SystemTime::now();
            return Some(move_distance);
        }
        return None;
    }

    pub fn get_move_right_left_value(&mut self) -> Option<f64> {
        if self.move_right == true && self.move_left == true {
            self.move_right_last_time = SystemTime::now();
            self.move_left_last_time = SystemTime::now();
            return None;
        }
        if self.move_right == true && self.move_left == false {
            let move_distance =
                -(self.move_right_last_time.elapsed().unwrap().as_nanos() as f64 / 50000000.0);
            self.move_right_last_time = SystemTime::now();
            return Some(move_distance);
        }
        if self.move_left == true && self.move_right == false {
            let move_distance =
                self.move_left_last_time.elapsed().unwrap().as_nanos() as f64 / 50000000.0;
            self.move_left_last_time = SystemTime::now();
            return Some(move_distance);
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    // #[test]
    // fn move_() {
    //     let move_handler = MoveHandler::new();
    // }
}
