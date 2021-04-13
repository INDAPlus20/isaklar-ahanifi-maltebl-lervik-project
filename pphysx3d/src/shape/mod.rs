use kiss3d::nalgebra::{self as na, Point3, Vector2};
use na::Vector3;
mod tests;
struct Particle {
    position: Point3<f32>,
    velocity: Vector3<f32>,
}

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

struct Boundary {
    size: Vector3<f32>,
    position: Point3<f32>,
}

impl Boundary {
    fn contains(&self, point: Vector3<f32>) -> bool {
        todo!()
    }

    fn hit_point(&self, ray: Ray) -> Point3<f32> {
        todo!()
    }
}
pub trait Shape {
    fn get_position(&self) -> Point3<f32>;

    fn bounding_box(&self) -> Boundary;
}

struct Cube {
    size: Vector3<f32>,
    position: Point3<f32>,
    //rotation: f32,
    //rotation_velocity:
}

impl Shape for Cube {
    fn get_position(&self) -> Point3<f32> {
        todo!()
    }

    fn bounding_box(&self) -> Boundary {
        todo!()
    }
}

struct Sphere {
    radius: f32,
    position: Point3<f32>,
}

struct GameObject {
    shape: Box<dyn Shape>,
    velocity: Vector3<f32>,
    //texture:
    mass: f32,
}
