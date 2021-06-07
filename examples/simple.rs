use game_engine_3d::*;

use engine::Engine;
use graph::Coordinate;
use map_element::{Door, DoorType, DoorVelocity, MapElement, Point, Rectangle, WallMap};
use player_utils::{Angle, Player, Radians};

fn main() {
    let path_to_map = "test_resources/map-simple.png";
    let resolution = Size {
        width: 1280.0,
        height: 720.0,
    };
    let number_of_rays = 2000;
    let player = Player::new(
        Angle {
            start: Radians::new(std::f64::consts::PI),
            end: Radians::new(3.0 * std::f64::consts::PI / 2.0),
        },
        Coordinate { x: 63.0, y: 90.0 },
        number_of_rays,
    );
    let vertical_angle_value = Radians::new(std::f64::consts::PI * 0.375);
    let wall_height = 5.0;

    match WallMap::new(path_to_map) {
        Ok(map) => {
            let map_elements: Vec<Box<dyn MapElement>> = vec![
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 55, y: 43 },
                        point_b: Point { x: 76, y: 45 },
                    },
                    DoorVelocity::VeryFast,
                    DoorType::Horizontal,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 55, y: 25 },
                        point_b: Point { x: 76, y: 27 },
                    },
                    DoorVelocity::VeryFast,
                    DoorType::Horizontal,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 22, y: 36 },
                        point_b: Point { x: 39, y: 38 },
                    },
                    DoorVelocity::Fast,
                    DoorType::Horizontal,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 89, y: 36 },
                        point_b: Point { x: 105, y: 38 },
                    },
                    DoorVelocity::Fast,
                    DoorType::Horizontal,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 113, y: 16 },
                        point_b: Point { x: 115, y: 32 },
                    },
                    DoorVelocity::Fast,
                    DoorType::Vertical,
                    None,
                )),
                Box::new(map),
            ];
            let map = Map {
                width: 130,
                height: 117,
            };

            let mut engine = Engine::new(
                resolution,
                player,
                vertical_angle_value,
                wall_height,
                map_elements,
                map,
            );
            engine.start();
        }
        Err(_) => {}
    }
}
