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

// struct Cube {
//     size: Vector3<f32>,
//     position: Point3<f32>,
//     //rotation: f32,
//     //rotation_velocity:
// }

// impl Shape for Cube {
//     fn get_position(&self) -> Point3<f32> {
//         todo!()
//     }

//     fn bounding_box(&self) -> Boundary {
//         todo!()
//     }
// }

struct GameObject {
    shape: Box<dyn Shape>,    // The collider
    position: Isometry3<f32>, // includes a translation vector and a rotation part as an unit quaternion
    velocity: Vector3<f32>,
    //texture:
    mass: f32,
}
