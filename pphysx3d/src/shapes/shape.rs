use kiss3d::nalgebra::Point3;

use super::{bounding_volume::{BoundingSphere, AABB}, sphere::Sphere};

pub trait Shape {
    fn compute_aabb(&self) -> AABB;
    fn get_position(&self) -> Point3<f32>;
    fn compute_bounding_sphere(&self) -> BoundingSphere;
}

impl Shape for Sphere{
    fn compute_aabb(&self) -> AABB {
        self.aabb()
    }

    fn compute_bounding_sphere(&self) -> BoundingSphere {
        self.bounding_sphere()
    }

    fn get_position(&self) -> Point3<f32> {
        self.center
    }
}
