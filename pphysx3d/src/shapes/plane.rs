use kiss3d::nalgebra::{Isometry3, Point3, UnitVector3, Vector3};

use super::{bounding_volume::{AABB, BoundingSphere}, shape::Shape, sphere::Sphere};

pub struct Plane {
    normal: UnitVector3<f32>
}

impl Plane {
    pub fn new(normal: UnitVector3<f32>) -> Plane {
        Plane{
            normal
        }
    }

    /// Normalises the vector and constructs a new ´Plane´
    pub fn from_vector3(vector: Vector3<f32>) -> Plane {
        Plane {
            normal: UnitVector3::new_normalize(vector)
        }
    }

    /// Returns the normal of this plane
    pub fn normal(&self) -> &UnitVector3<f32> {
        return &self.normal
    }

}

impl Shape for Plane {
    fn compute_aabb(&self, _pos: &Isometry3<f32>) -> AABB{
        // Since a plane has infinite size, the AABB is a bit weird
        let half_max = f32::MAX/2.0;
        let maxs = Point3::new(half_max, half_max, half_max);    
        return AABB::new(-maxs, maxs);
    }
    fn compute_bounding_sphere(&self, _pos: &Isometry3<f32>) -> BoundingSphere{
        // Since a plane has infinite size, the BoundingSphere is a bit weird
        let center: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
        return BoundingSphere::new(f32::MAX/2.0, center)
    }   
    fn as_sphere(&self) -> Option<&Sphere>{
        return None;
    }
}