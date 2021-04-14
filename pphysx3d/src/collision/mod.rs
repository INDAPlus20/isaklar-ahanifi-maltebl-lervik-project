use crate::shapes::{bounding_volume::BoundingVolume, shape::Shape};

mod tests;


/// The broad phase, where we check for possible collisions using AABB. Returns collision pairs
pub fn broad_phase<T: Shape>(objects: &Vec<Box<T>>) -> Vec<(&Box<T>, &Box<T>)>{
    
    // This is a naive solution at O(n^2), the plan is to do a Dynamic Bounding Volume Tree
    let mut collisions: Vec<(&Box<T>, &Box<T>)> = Vec::with_capacity(objects.len()*objects.len());
    for current in objects {
        for test in objects {
            if current.compute_aabb().interects(&test.compute_aabb()){
                collisions.push((current, test));
            }
        }
    }

    return collisions;
}

// Checks collision pairs if they actually collide
pub fn narrow_phase<T: Shape>(pairs: Vec<(&Box<T>, &Box<T>)>) {
    
}