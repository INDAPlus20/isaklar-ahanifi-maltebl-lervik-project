use kiss3d::nalgebra::Isometry3;

use super::{
    bounding_volume::{BoundingSphere, AABB},
    sphere::Sphere,
};

pub trait Shape {
    fn compute_aabb(&self, pos: &Isometry3<f32>) -> AABB;
    fn compute_bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere;
    fn as_sphere(&self) -> Option<&Sphere>;
}
