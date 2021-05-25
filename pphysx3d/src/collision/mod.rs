use kiss3d::nalgebra::{Isometry3, Point3, Unit, UnitVector3, Vector3};

use crate::shapes::{plane::Plane, sphere::Sphere};

mod tests;

/// Collision check for two spheres with given translation
fn sphere_sphere(
    sphere_a: &Sphere,
    sphere_b: &Sphere,
    iso_a: &Isometry3<f32>,
    iso_b: &Isometry3<f32>,
) -> bool {
    // Check if sum of radiuses >= distance
    let diff: Vector3<f32> = iso_a.translation.vector - iso_b.translation.vector;
    let squared_distance: f32 = diff.norm_squared();
    let radiuses = sphere_a.radius + sphere_b.radius;

    if squared_distance <= radiuses * radiuses {
        return true;
    } else {
        return false;
    }
}

/// Collision check for a sphere and a plane
fn sphere_plane(
    sphere: &Sphere,
    plane: &Plane,
    iso_s: &Isometry3<f32>,
    iso_p: &Isometry3<f32>,
) -> bool {
    // Find the closest point (thank god for linalg)
    let normal: UnitVector3<f32> = iso_s.rotation * plane.normal();
    let dist_to_center = iso_p.translation.vector - iso_s.translation.vector;
    // project the distance vector onto the normal
    let proj: Vector3<f32> = normal.scale(normal.dot(&dist_to_center));
    let dist_squared = proj.norm_squared();
    if dist_squared <= sphere.radius * sphere.radius {
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
        manifold.normal = UnitVector3::new_normalize(distance.scale(-1.));
        manifold.depth = ((distance.norm() - radiuses) / 2.0f32).abs();
        let point_dist = sphere_a.radius - manifold.depth; // distance to contact point
        let contact_point: Point3<f32> =
            (iso_a.translation.vector + manifold.normal.scale(point_dist)).into();
        manifold.contacts.push(contact_point);

        return manifold;
    }

    /// Calculates the collision manifold between a sphere and a plane
    pub fn sphere_plane(
        sphere: &Sphere,
        plane: &Plane,
        iso_s: &Isometry3<f32>,
        iso_p: &Isometry3<f32>,
    ) -> CollisionManifold {
        let mut manifold = CollisionManifold::new();

        // project the distance between the sphere center and plane center onto the plane normal
        let normal: UnitVector3<f32> = iso_p.rotation * plane.normal();
        let dist_to_center = iso_p.translation.vector - iso_s.translation.vector;
        let proj: Vector3<f32> = normal.scale(normal.dot(&dist_to_center));
        let dist_squared = proj.norm_squared();

        // check if colliding
        if dist_squared <= sphere.radius * sphere.radius {
            manifold.colliding = true;
        } else {
            // return the default manifold
            return manifold;
        }

        manifold.depth = sphere.radius - normal.dot(&dist_to_center);
        let contact_point: Point3<f32> = Point3::from(iso_s.translation.vector + proj);
        manifold.contacts = vec![contact_point];
        manifold.normal = normal;

        return manifold;
    }
}
