use core::f32;

use crate::shapes::bounding_volume::AABB;
use kiss3d::nalgebra::{self as na, Point3, Vector3};

use super::bounding_volume::BoundingSphere;

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius:f32,center:Point3<f32>)-> Sphere {
        Sphere{
            center,
            radius,
        }
    }
    pub fn aabb(&self) -> AABB {
        AABB::new(
            (self.center + Vector3::repeat(-self.radius)).into(),
            (self.center + Vector3::repeat(self.radius)).into(),
        )
    }
    pub fn bounding_sphere(&self) -> BoundingSphere{
        BoundingSphere::new(self.radius, self.center.into())
    }
}
