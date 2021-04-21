use kiss3d::nalgebra::{self as na, Isometry3, Point3, Vector2};
use na::Vector3;

use self::{shape::Shape, sphere::Sphere};
mod bounding_volume;
mod shape;
mod sphere;
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

struct GameObject {
    shape: Box<dyn Shape>,    // The collider
    position: Isometry3<f32>, // includes a translation vector and a rotation part as an unit quaternion
    velocity: Vector3<f32>,
    //texture:
    mass: f32,
}
