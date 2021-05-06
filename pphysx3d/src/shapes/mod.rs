use kiss3d::nalgebra::{self as na, Isometry3, Point3, Vector2};
use na::Vector3;

use self::{shape::Shape, sphere::Sphere};
pub mod bounding_volume;
pub mod shape;
pub mod sphere;
pub mod plane;
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

pub struct GameObject {
    pub shape: Box<dyn Shape>,    // The collider
    pub position: Isometry3<f32>, // includes a translation vector and a rotation part as an unit quaternion
    pub velocity: Vector3<f32>,
    //texture:
    pub mass: f32,
}
