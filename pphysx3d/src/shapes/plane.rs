use kiss3d::nalgebra::{Isometry3, Point3, UnitVector3, Vector3};

use super::{
    bounding_volume::{BoundingSphere, AABB},
    ray::Ray,
    raycast::{RayCast, RayCastResult},
    shape::Shape,
    sphere::Sphere,
};
///The shape of a plane, defined by its normal
pub struct Plane {
    normal: UnitVector3<f32>,
}

impl Plane {
    pub fn new(normal: UnitVector3<f32>) -> Plane {
        Plane { normal }
    }

    /// Normalises the vector and constructs a new ´Plane´
    pub fn from_vector3(vector: Vector3<f32>) -> Plane {
        Plane {
            normal: UnitVector3::new_normalize(vector),
        }
    }

    /// Returns the normal of this plane
    pub fn normal(&self) -> &UnitVector3<f32> {
        return &self.normal;
    }
}

impl Shape for Plane {
    fn compute_aabb(&self, _pos: &Isometry3<f32>) -> AABB {
        // Since a plane has infinite size, the AABB is a bit weird
        let half_max = f32::MAX / 2.0;
        let maxs = Point3::new(half_max, half_max, half_max);
        return AABB::new(-maxs, maxs);
    }
    fn compute_bounding_sphere(&self, _pos: &Isometry3<f32>) -> BoundingSphere {
        // Since a plane has infinite size, the BoundingSphere is a bit weird
        let center: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
        return BoundingSphere::new(f32::MAX / 2.0, center);
    }
    fn as_sphere(&self) -> Result<&Sphere, ()> {
        return Err(());
    }

    fn as_plane(&self) -> Result<&Plane, ()> {
        Ok(&self)
    }
}

impl RayCast for Plane {
    /// Does a raycast test on `self` with transform `pos: &Isometry3<f32>`
    fn ray_cast(&self, pos: &Isometry3<f32>, ray: &Ray) -> RayCastResult {
        let mut result = RayCastResult::new();
        let normal: UnitVector3<f32> = pos.rotation * self.normal;
        // transform ray origin to local plane coords so that plane equation: N*P = 0;
        let local_origin = ray.origin() - pos.translation.vector;
        let nd = normal.dot(ray.direction());
        let pn = normal.dot(&local_origin.coords);

        // the dot of plane normal and direction have to be negative
        if nd >= 0.0 {
            return result;
        }
        let toi = (-pn) / nd; // math

        // negative time of impact means hit behind plane (which shouldn't happen according to above check??)
        if toi < 0.0 {
            return result;
        }

        result.contact_point = ray.origin() + ray.direction().scale(toi);
        result.normal = self.normal;
        result.hit = true;
        result.distance = toi;

        return result;
    }
}
