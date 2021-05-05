use crate::collision::*;
use crate::shapes::{bounding_volume::BoundingVolume, GameObject};
use kiss3d::nalgebra::Translation;

mod tests;
pub struct PhysicsScene {
    objects: Vec<GameObject>,
}

impl PhysicsScene {
    pub fn new() -> PhysicsScene {
        PhysicsScene {
            objects: Vec::new(),
        }
    }

    /// Add a GameObject to the scene
    pub fn add(&mut self, object: GameObject) {
        self.objects.push(object);
    }

    /// Remove a GameObject by index from the scene
    pub fn remove(&mut self, index: usize) {
        self.objects.remove(index);
    }

    /// Updates the physics in the scene, such as collisions
    pub fn update(&mut self, time_step: f32) {
        // Physics loop

        // Detect collisions
        let collision_pairs = broad_phase(&self.objects);
        let manifolds = narrow_phase(&self.objects, &collision_pairs);

        // Resolve Collisions & Apply impulse

        // update positions
        self.update_positions(time_step);
    }

    pub fn get_objects(&self) -> &[GameObject] {
        &self.objects
    }

    /// Updates the positions according to their linear velocity, with timestep `time`
    fn update_positions(&mut self, time_step: f32) {
        for object in &mut self.objects {
            object.position = Translation::from(object.velocity * time_step) * object.position;
            //object.position.translation.vector + object.velocity*time_step;
        }
    }
}

/// The broad phase, where we check for possible collisions using AABB.
/// Returns indices for collision pairs
fn broad_phase(objects: &Vec<GameObject>) -> Vec<(usize, usize)> {
    // This is a naive solution at O(n^2), the plan is to do a Bounding Volume Tree at some point
    let mut collisions: Vec<(usize, usize)> = Vec::with_capacity(objects.len() * objects.len());
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
pub fn narrow_phase(
    objects: &Vec<GameObject>,
    pairs: &Vec<(usize, usize)>,
) -> Vec<CollisionManifold> {
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
