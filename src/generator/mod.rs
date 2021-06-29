mod object_generator;
mod point_generator;
mod polygon;
mod polygon_generator;

pub use object_generator::ObjectGenerator;
pub use point_generator::PointGenerator;
pub use polygon::Polygon;
pub use polygon_generator::PolygonGenerator;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use object_generator::MockObjectGenerator;
        use point_generator::MockPointGenerator;
        use polygon_generator::MockPolygonGenerator;
    }
}
