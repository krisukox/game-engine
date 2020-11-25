use game_engine_3d::*;

use engine::Engine;
use graph::Coordinate;
use player_utils::{Angle, Player, Radians};

fn main() {
    let path_to_map = "test_resources/map.png";
    let resolution = Size {
        width: 1280.0,
        height: 720.0,
    };
    let number_of_rays = 2000;
    let player = Player::new(
        Angle {
            start: Radians(std::f64::consts::PI),
            end: Radians(3.0 * std::f64::consts::PI / 2.0),
        },
        Coordinate { x: 27.0, y: 9.0 },
        number_of_rays,
    );
    let vertical_angle_value = Radians(std::f64::consts::PI * 0.375);
    let wall_height = 5.0;

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
