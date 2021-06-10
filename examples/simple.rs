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
            start: Radians::new(std::f64::consts::PI * 5.0 / 4.0),
            end: Radians::new(std::f64::consts::PI * 7.0 / 4.0),
        },
        Coordinate { x: 66.0, y: 58.0 },
        number_of_rays,
    );
    let vertical_angle_value = Radians::new(std::f64::consts::PI * 0.375);
    let wall_height = 5.0;
    let render_threads_amount = 3;

    match WallMap::new(path_to_map, None) {
        Ok(map) => {
            let map_elements: Vec<Box<dyn MapElement>> = vec![
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 56, y: 43 },
                        point_b: Point { x: 75, y: 45 },
                    },
                    DoorVelocity::VeryFast,
                    DoorType::Horizontal,
                    None,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 56, y: 25 },
                        point_b: Point { x: 75, y: 27 },
                    },
                    DoorVelocity::VeryFast,
                    DoorType::Horizontal,
                    None,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 23, y: 36 },
                        point_b: Point { x: 38, y: 38 },
                    },
                    DoorVelocity::Fast,
                    DoorType::Horizontal,
                    None,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 90, y: 36 },
                        point_b: Point { x: 104, y: 38 },
                    },
                    DoorVelocity::Fast,
                    DoorType::Horizontal,
                    None,
                    None,
                )),
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 113, y: 17 },
                        point_b: Point { x: 115, y: 31 },
                    },
                    DoorVelocity::Fast,
                    DoorType::Vertical,
                    None,
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
                vertical_angle_value,
                wall_height,
                map,
                player,
                map_elements,
                render_threads_amount,
            );
            engine.start();
        }
        Err(_) => {}
    }
}
