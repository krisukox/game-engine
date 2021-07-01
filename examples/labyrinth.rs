use game_engine_3d::*;
use map_element::*;
use std::path::Path;

fn main() {
    // let path = Path::new("examples/labyrinth.png");
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/labyrinth.png");
    let resolution = Size {
        width: 1280.0,
        height: 720.0,
    };
    let number_of_rays = 10000;
    let player = Player::new(
        Angle {
            start: Radians::new(std::f64::consts::PI * 3.0 / 4.0),
            end: Radians::new(std::f64::consts::PI * 5.0 / 4.0),
        },
        Coordinate { x: 495.0, y: 303.0 },
        number_of_rays,
    );
    let vertical_angle_value = Radians::new(std::f64::consts::PI * 0.375);
    let wall_height = 5.0;
    let render_threads_amount = 3;

    match WallMap::new(&path, None) {
        Ok(wall_map) => {
            let map = wall_map.get_map();
            let map_elements: Vec<Box<dyn MapElement>> = vec![Box::new(wall_map)];

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
