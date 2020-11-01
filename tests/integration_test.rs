// extern crate gfx_device_gl;
// extern crate gfx_graphics;
// extern crate glutin_window;
// extern crate graphics;
// extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{Size, WindowSettings};
// use piston_window::Size;

use game_engine_3d::*;
use map::Map;
use object_generator::ObjectGenerator;
use player_utils::Player;
use point_generator::PointGenerator;
use polygon_generator::PolygonGenerator;
#[test]
fn cos() {
    // let mut window: PistonWindow = WindowSettings::new(
    //     "Hello World!",
    //     Size {
    //         width: 800,
    //         height: 600,
    //     },
    // )
    // .build()
    // .unwrap();
    // // while let Some(e) = window.next() {
    // //     window.draw_2d(&e, |c, g| {
    // //         clear([0.5, 0.5, 0.5, 1.0], g);
    // //         rectangle(
    // //             [1.0, 0.0, 0.0, 1.0],     // red
    // //             [0.0, 0.0, 100.0, 100.0], // rectangle
    // //             c.transform,
    // //             g,
    // //         );
    // //     });
    // // }
    // let mut rays: Vec<graph::LinearGraph> = Vec::new();
    // let mut polygon_generator = PolygonGenerator {
    //     point_generator: PointGenerator {
    //         resolution: Size {
    //             width: 800,
    //             height: 600,
    //         },
    //         half_vertical_angle_value: player_utils::Radians(std::f64::consts::PI / 2.0),
    //         wall_height: 5.0,
    //     },
    // };
    // let mut radians = 0.0;
    // while radians < std::f64::consts::PI * 2.0 {
    //     rays.push(graph::LinearGraph::from_radians(radians));
    //     radians += 0.02;
    // }
    // if let Ok(map) = Map::new("test_resources/map.png") {
    //     let object_generator = ObjectGenerator {
    //         map,
    //         rays: rays.clone(),
    //         object_generator: polygon_generator,
    //     };
    //     // let angle = player_utils::Angle {
    //     //     start: player_utils::Radians(0.0),
    //     //     end: player_utils::Radians(std::f64::consts::PI / 2.0),
    //     // };
    //     // println!("LEN: {}", angle.get_rays_angle_range(rays.len()).len());

    //     // while let Some(e) = window.next() {
    //     //     window.draw_2d(&e, |c, g| {
    //     //         let transform = c.transform.flip_v().trans(0.0, -300.0);
    //     //         // clear([0.5, 0.5, 0.5, 1.0], g);
    //     //         let position = graph::Coordinate { x: 27.0, y: 26.0 };
    //     //         let angle = player_utils::Angle {
    //     //             start: player_utils::Radians(1.0),
    //     //             end: player_utils::Radians(3.0),
    //     //         };
    //     //         let polygons = object_generator.generate_polygons(
    //     //             position,
    //     //             angle.get_rays_angle_range(rays.len())[0].clone(),
    //     //             angle,
    //     //         );
    //     //         // polygon(
    //     //         //     [1.0, 0.0, 0.0, 1.0],
    //     //         //     &[[0.0, -200.0], [0.0, 200.0], [300.0, 300.0], [300.0, -300.0]],
    //     //         //     transform,
    //     //         //     g,
    //     //         // );
    //     //         for polygon_ in polygons {
    //     //             polygon([1.0, 0.0, 0.0, 1.0], &polygon_, transform, g);
    //     //         }
    //     //         // polygon([1.0, 0.0, 0.0, 1.0], , transform, g)
    //     //         // rectangle(
    //     //         //     [1.0, 0.0, 0.0, 1.0],     // red
    //     //         //     [0.0, 0.0, 100.0, 100.0], // rectangle
    //     //         //     c.transform,
    //     //         //     g,
    //     //         // );
    //     //     });
    //     // }
    // }
    assert!(true);
}

use crate::engine::Engine;

#[test]
fn cos1() {
    let path_to_map = "test_resources/map.png";
    let resolution = Size {
        width: 800.0,
        height: 600.0,
    };
    let number_of_rays = 400;
    let player = Player::new(
        player_utils::Angle {
            start: player_utils::Radians(1.0),
            end: player_utils::Radians(3.0),
        },
        graph::Coordinate { x: 27.0, y: 26.0 },
        number_of_rays,
    );
    let vertical_angle_value = player_utils::Radians(std::f64::consts::PI);
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
