use core::f32;

use crate::shapes::bounding_volume::AABB;
use kiss3d::nalgebra::{self as na, Isometry3, Point3, Vector3};

use super::{bounding_volume::BoundingSphere, shape::Shape};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius }
    }
    pub fn aabb(&self, pos: &Isometry3<f32>) -> AABB {
        let center = Point3::from(pos.translation.vector);
        AABB::new(
            center + Vector3::repeat(-self.radius), //
            center + Vector3::repeat(self.radius),
        )
    }
    pub fn bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere {
        let center = Point3::from(pos.translation.vector);
        BoundingSphere::new(self.radius, center)
    }
}

impl Shape for Sphere {
    fn compute_aabb(&self, pos: &Isometry3<f32>) -> AABB {
        self.aabb(&pos)
    }
    fn compute_bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere {
        self.bounding_sphere(pos)
    }

    fn as_sphere(&self) -> Result<&Sphere, ()> {
        Ok(self)
    }
}
