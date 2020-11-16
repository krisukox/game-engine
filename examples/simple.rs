extern crate game_engine_3d;

use crate::engine::Engine;
use game_engine_3d::*;
use piston::window::Size;
use player_utils::Player;

fn main() {
    let path_to_map = "test_resources/map.png";
    let resolution = Size {
        width: 1280.0,
        height: 720.0,
    };
    let number_of_rays = 2000;
    let player = Player::new(
        player_utils::Angle {
            start: player_utils::Radians(1.0 * std::f64::consts::PI / 4.0),
            end: player_utils::Radians(3.0 * std::f64::consts::PI / 4.0),
        },
        graph::Coordinate { x: 35.0, y: 11.0 },
        number_of_rays,
    );
    let vertical_angle_value = player_utils::Radians(std::f64::consts::PI);
    let wall_height = 7.0;

    if let Ok(mut engine) = Engine::new(
        path_to_map,
        resolution,
        player,
        vertical_angle_value,
        wall_height,
    ) {
        engine.start()
    }
}
