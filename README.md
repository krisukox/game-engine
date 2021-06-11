# game-engine

[![Build Status](https://github.com/krisukox/game-engine/workflows/Build/badge.svg)](https://github.com/krisukox/game-engine/actions)
[![codecov](https://codecov.io/gh/krisukox/game-engine/branch/master/graphs/badge.svg?token=H1GXCQQ3YG)](https://codecov.io/gh/krisukox/game-engine/tree/master/src)

Game engine 3D uses [piston2d-graphics](https://github.com/pistondevelopers/graphics) with [opengl backend](https://github.com/PistonDevelopers/opengl_graphics) to draw 2d objects and gather mouse and keyboard event. All 3D structures are rendered by game-engine-project using ray casting. Rendering can be devided into multiple threads (up to 4 threads). 



Game engine 


Unit tests in the project are made with [Mockall](https://docs.rs/mockall/0.9.1/mockall/)
Code coverage reporting tool used in the project [Tarpaulin](https://github.com/xd009642/tarpaulin)


Movement

You can move a player using WASD keys and mouse


Examples

```
cargo run --example simple --release
```
Simple example  with own prepared map-simple.png and placed Doors


```
cargo run --example simple --release
```
example with the labyrinth image


## Getting Started
```
cargo new --bin start-project
cd start-project
```

Cargo.toml
```
[package]
name = "start-project"
version = "0.1.0"
authors = ["Krzysztof Naglik <krzysztofnaglik96@gmail.com>"]
edition = "2018"

[dependencies]
game_engine_3d = { git = "https://github.com/krisukox/game-engine" }
```

src/main.rs
```
use game_engine_3d::*;
use map_element::*;
use std::path::Path;

fn main() {
    let path = Path::new("path-to-the-image/image.png");
    let resolution = Size {
        width: 1280.0,
        height: 720.0,
    };
    let number_of_rays = 2000;
    let player = Player::new(
        Angle {
            start: Radians::new(std::f64::consts::PI * 3.0 / 4.0),
            end: Radians::new(std::f64::consts::PI * 5.0 / 4.0),
        },
        Coordinate { x: 10.0, y: 10.0 },
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
```

### Engine

Engine parameters:

| Name | Description | Type | Possible / Optimal values |
|------|-----------|--------|---------------------------|
| resolution | resolution of the screen | Size |  | 
| vertical_angle_value | vertical angle of player view | [Radians](src/player_utils/radians.rs) | [0.25pi, 0.6pi] |
| wall_height | height of walls | f64 | [3, 10] |
| map | area where player is moving and map elements are placed | [Map](src/map.rs) |  |
| player | describes player start position, view angle and number of rays | [Player](src/player_utils/player.rs)  |  |
| map_elements | map elements rendered which will be rendered | Vec\<[MapElement](src/map_element/map_element.rs)\> | [WallMap](src/map_element/wall_map.rs), [Door](src/map_element/door.rs) |
| render_threads_amount | amounts of the render threads (range ) | i64 | [1, 4] |



### Map elements

* [`WallMap`](src/map_element/wall_map.rs) - WallMap is a structure that describes where Walls are placed on the game area. It takes path to the image that shows walls locations (top view). Image has to be black(grey) and white. Range of the color where walls are located is RGB[0,0,0] - RGB[99, 99, 99]. It takes also color of the Walls. Default color is orange.

* [`Door`](src/map_element/door.rs) - Door structure describes where door is located, opening direction and color of the door. 


### Other types

* [`Color`](src/map_element/color.rs) - 