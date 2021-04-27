use std::any::Any;

use kiss3d::nalgebra::{
    abs, distance_squared, Isometry3, Point3, RealField, Unit, UnitVector3, Vector3,
};

use crate::shapes::{bounding_volume::BoundingVolume, shape::Shape, sphere::Sphere, GameObject};

mod tests;

/// The broad phase, where we check for possible collisions using AABB. Returns collision pairs
pub fn broad_phase<T: Shape>(objects: &Vec<Box<T>>) -> Vec<(&Box<T>, &Box<T>)> {
    // This is a naive solution at O(n^2), the plan is to do a Dynamic Bounding Volume Tree at some point
    let mut collisions: Vec<(&Box<T>, &Box<T>)> = Vec::with_capacity(objects.len() * objects.len());
    for current_i in 0..objects.len() {
        let current = &objects[current_i];
        for test_i in current_i..objects.len() {
            let test = &objects[test_i];
            if current_i != test_i && current.compute_aabb().interects(&test.compute_aabb()) {
                collisions.push((current, test));
            }
        }
    }

    return collisions;
}

/// Checks collision pairs if they actually collide
pub fn narrow_phase<T: Shape>(pairs: &mut Vec<(&Box<T>, &Box<T>)>) {}

/// Collision check for two spheres
pub fn sphere_sphere(
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

/// Contains the necessary information to resolve a coliision
pub struct CollisionManifold {
    colliding: bool,
    normal: Vector3<f32>,
    depth: f32,
    contacts: Vec<Point3<f32>>,
}

/// Calculates the collision manifold for two colliding objects
pub fn calculate_collision_manifold(obj_a: &GameObject, obj_b: &GameObject) {}
