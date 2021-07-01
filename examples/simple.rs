use game_engine_3d::*;
use map_element::*;
use std::path::Path;

fn main() {
    // let path = Path::new("examples/map-simple.png");
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/map-simple.png");
    let resolution = Size {
        width: 1280.0,
        height: 720.0,
    };
    let number_of_rays = 10000;
    let player = Player::new(
        Angle {
            start: Radians::new(std::f64::consts::PI * 5.0 / 4.0),
            end: Radians::new(std::f64::consts::PI * 7.0 / 4.0),
        },
        Coordinate { x: 66.0, y: 84.0 },
        number_of_rays,
    );
    let vertical_angle_value = Radians::new(std::f64::consts::PI * 0.375);
    let wall_height = 5.0;
    let render_threads_amount = 3;

    match WallMap::new(&path, None) {
        Ok(wall_map) => {
            let map = wall_map.get_map();
            let map_elements: Vec<Box<dyn MapElement>> = vec![
                Box::new(Door::new(
                    Rectangle {
                        point_a: Point { x: 55, y: 43 },
                        point_b: Point { x: 76, y: 45 },
                    },
                    DoorVelocity::VeryFast,
                    DoorType::Horizontal,
                    None,
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
                    None,
                )),
                Box::new(wall_map),
            ];

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
        Err(_) => {
            println!("file not found");
        }
    }
}
