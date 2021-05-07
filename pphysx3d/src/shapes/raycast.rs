use kiss3d::nalgebra::{Isometry3, Point3, Vector3};

use super::ray::Ray;

pub struct RayCastResult {
    pub distance: f32,
    pub contact_point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub hit: bool
}

impl RayCastResult {
    pub fn new() -> RayCastResult {
        RayCastResult {
            distance: -1.0,
            contact_point: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            hit: false
        }
    }
}

pub trait RayCast {
    /// Does a raycast test on `self` with transform `pos: &Isometry3<f32>`
    fn ray_cast(&self, pos: &Isometry3<f32>, ray: &Ray) -> RayCastResult;
}



