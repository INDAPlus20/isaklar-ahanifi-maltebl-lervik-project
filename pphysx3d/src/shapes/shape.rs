use kiss3d::nalgebra::Isometry3;

use super::{
    bounding_volume::{BoundingSphere, AABB},
    cube::Cube,
    sphere::Sphere,
};

pub trait Shape {
    fn compute_aabb(&self, pos: &Isometry3<f32>) -> AABB;
    fn compute_bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere;
    fn as_sphere(&self) -> Result<&Sphere, ()>;
    fn as_cube(&self) -> Result<&Cube, ()>;
}
