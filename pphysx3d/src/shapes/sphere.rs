use super::{
    bounding_volume::BoundingSphere,
    cube::Cube,
    ray::Ray,
    raycast::{RayCast, RayCastResult},
    shape::Shape,
};
use crate::shapes::bounding_volume::AABB;
use core::f32;
use kiss3d::nalgebra::{Isometry3, Point, Point3, UnitVector3, Vector3};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius }
    }
    /// Returns the axis-aligned bounding box of the sphere with the position given by the  ```pos: &Isometry3<f32>```
    pub fn aabb(&self, pos: &Isometry3<f32>) -> AABB {
        let center = Point3::from(pos.translation.vector);
        AABB::new(
            center + Vector3::repeat(-self.radius),
            center + Vector3::repeat(self.radius),
        )
    }
    /// Returns the bounding sphere of the sphere with the position given by the  ```pos: &Isometry3<f32>```
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

    fn as_cube(&self) -> Result<&Cube, ()> {
        Err(())
    }

    fn as_plane(&self) -> Result<&super::plane::Plane, ()> {
        Err(())
    }
}

impl RayCast for Sphere {
    /// Does a raycast test on `self` with transform `pos: &Isometry3<f32>`
    fn ray_cast(&self, pos: &Isometry3<f32>, ray: &Ray) -> RayCastResult {
        let mut result = RayCastResult::new();
        let distance_to_center: Vector3<f32> = Point3::from(pos.translation.vector) - ray.origin();
        // project the distance vector to the direction of the ray
        let proj: Vector3<f32> = ray
            .direction()
            .scale(ray.direction().dot(&distance_to_center));
        let distance_to_proj: Vector3<f32> = proj - pos.translation.vector;
        if self.radius * self.radius < distance_to_proj.norm_squared() {
            return result; // not intersecting
        }
        let penetration_depth =
            (self.radius * self.radius - distance_to_proj.norm_squared()).sqrt();
        let mut toi = proj.norm() - penetration_depth;
        // Reverse time of impact if ray starts inside sphere
        if distance_to_center.norm_squared() < self.radius * self.radius {
            toi += penetration_depth * 2.0;
        }

        result.distance = toi;
        result.hit = true;
        result.contact_point = ray.origin() + ray.direction().scale(toi);
        result.normal =
            UnitVector3::new_normalize(result.contact_point - Point3::from(pos.translation.vector));
        return result;
    }
}
