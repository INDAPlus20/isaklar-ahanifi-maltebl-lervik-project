use kiss3d::nalgebra::{Isometry3, Point3, Unit, UnitVector3, Vector3};

use crate::shapes::{bounding_volume::BoundingVolume, sphere::Sphere, GameObject};

mod tests;

/// Collision check for two spheres with given translation
fn sphere_sphere(
    sphere_a: &Sphere,
    sphere_b: &Sphere,
    iso_a: &Isometry3<f32>,
    iso_b: &Isometry3<f32>,
) -> bool {
    // Check if sum of radiuses >= distance
    let diff: Vector3<f32> = (iso_a.translation.vector - iso_b.translation.vector);
    let squared_distance: f32 = diff.norm_squared();
    let radiuses = sphere_a.radius + sphere_b.radius;

    if squared_distance <= radiuses * radiuses {
        return true;
    } else {
        return false;
    }
}
#[derive(Debug, PartialEq)]
/// Contains the necessary information to resolve a coliision
pub struct CollisionManifold {
    pub colliding: bool,
    pub normal: Unit<Vector3<f32>>,
    pub depth: f32,
    pub contacts: Vec<Point3<f32>>,
}

impl CollisionManifold {
    pub fn new() -> CollisionManifold {
        CollisionManifold {
            colliding: false,
            normal: UnitVector3::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
            depth: f32::MAX,
            contacts: Vec::with_capacity(10),
        }
    }

    /// Calculates the collision manifold between two spheres
    pub fn sphere_sphere(
        sphere_a: &Sphere,
        sphere_b: &Sphere,
        iso_a: &Isometry3<f32>,
        iso_b: &Isometry3<f32>,
    ) -> CollisionManifold {
        let mut manifold = CollisionManifold::new();

        let distance: Vector3<f32> = iso_b.translation.vector - iso_a.translation.vector;
        let squared_distance: f32 = distance.norm_squared();
        let radiuses = sphere_a.radius + sphere_b.radius;

        // check if colliding
        if squared_distance > radiuses * radiuses {
            return manifold;
        }

        manifold.colliding = true;
        manifold.normal = UnitVector3::new_normalize(distance);
        manifold.depth = ((distance.norm() - radiuses) / 2.0f32).abs();
        let point_dist = sphere_a.radius - manifold.depth; // distance to contact point
        let contact_point: Point3<f32> =
            (iso_a.translation.vector + manifold.normal.scale(point_dist)).into();
        manifold.contacts.push(contact_point);

        return manifold;
    }
}
