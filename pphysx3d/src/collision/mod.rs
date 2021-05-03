use kiss3d::nalgebra::{Isometry3, Point3, Unit, UnitVector3, Vector3};

use crate::shapes::{bounding_volume::BoundingVolume, sphere::Sphere, GameObject};

mod tests;

/// The broad phase, where we check for possible collisions using AABB.
/// Returns collision pairs
pub fn broad_phase(objects: &Vec<GameObject>) -> Vec<(usize, usize)> {
    // This is a naive solution at O(n^2), the plan is to do a Bounding Volume Tree at some point
    let mut collisions: Vec<(usize, usize)> =
        Vec::with_capacity(objects.len() * objects.len());
    for current_i in 0..objects.len() {
        let current = &objects[current_i];
        for test_i in current_i..objects.len() {
            let test = &objects[test_i];
            if current_i != test_i
                && current
                    .shape
                    .compute_aabb(&current.position)
                    .interects(&test.shape.compute_aabb(&current.position))
            {
                collisions.push((current_i, test_i));
            }
        }
    }

    return collisions;
}

/// Calculates collision manifolds for the given collision pairs. 
/// Returns a list of manifolds in the same order as `pairs`
pub fn narrow_phase(objects: &Vec<GameObject>, pairs: &Vec<(usize, usize)>) -> Vec<CollisionManifold> {
    let mut manifolds: Vec<CollisionManifold> = Vec::with_capacity(pairs.len());
    for (obj_1, obj_2) in pairs {
        let obj_1 = &objects[*obj_1];
        let obj_2 = &objects[*obj_2];
        // pattern-match the specific collision
        if let (Some(sph_1), Some(sph_2)) = (obj_1.shape.as_sphere(), obj_2.shape.as_sphere()) {
            let manifold =
                CollisionManifold::sphere_sphere(&sph_1, &sph_2, &obj_1.position, &obj_2.position);
            manifolds.push(manifold);
        }
    }
    return manifolds;
}

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
    colliding: bool,
    normal: Unit<Vector3<f32>>,
    depth: f32,
    contacts: Vec<Point3<f32>>,
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
