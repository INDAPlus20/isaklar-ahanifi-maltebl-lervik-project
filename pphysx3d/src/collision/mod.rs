use kiss3d::nalgebra::distance_squared;

use crate::shapes::{bounding_volume::BoundingVolume, shape::Shape, sphere::Sphere};

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
pub fn narrow_phase<T: Shape>(pairs: &mut Vec<(&Box<T>, &Box<T>)>) {

}

/// Collision check for two spheres
pub fn sphere_sphere(s_1: &Sphere, s_2: &Sphere) -> bool {
    // Check if sum of radiuses >= distance
    let squared_distance = distance_squared(&s_1.center, &s_2.center);

    let radiuses = s_1.radius + s_2.radius;
    
    if squared_distance <= radiuses * radiuses {
        return true;
    } else {
        return false;
}
}
