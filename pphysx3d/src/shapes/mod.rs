use kiss3d::nalgebra::{self as na, Point3};
use na::Vector3;

pub mod bounding_volume;
pub mod cube;
pub mod plane;
pub mod shape;
pub mod sphere;
mod tests;
mod utils;

struct Particle {
    position: Point3<f32>,
    velocity: Vector3<f32>,
}

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}
