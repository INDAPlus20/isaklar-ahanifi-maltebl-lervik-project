use crate::collision::*;
use crate::shapes::{bounding_volume::BoundingVolume, GameObject};
use kiss3d::nalgebra::{Vector3, Translation};
use std::cmp::min;

mod tests;
pub struct PhysicsScene {
    objects: Vec<GameObject>,
}

impl PhysicsScene {
    pub fn new() -> PhysicsScene {
        PhysicsScene {
            objects: vec![],
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

    pub fn objects(&self) -> &[GameObject] {
        &self.objects
    }

    /// Updates the physics in the scene, such as collisions
    pub fn update(&mut self, time_step: f32) {
        // Physics loop

        // Detect collisions
        let collision_pairs = broad_phase(&self.objects);
        let manifolds = narrow_phase(&self.objects, &collision_pairs);

        // Resolve Collisions & Apply impulse + friction
            // (BAD CODE THAT NEEDS FIXING TO COMPILE :D)
        for (i,manifold) in manifolds.iter().enumerate() {
            if manifold.colliding {
                // COLLISION: 

                //&self.objects[i].add_force(Vector3::new(0., 0., 0.)); // this would be how to add new forces but we don't do that right here
                let index = &collision_pairs[i];
                let object_1 = &self.objects[index.0];
                let object_2 = &self.objects[index.1];
                // Calculate variables for readability: (unnecessary?)
                //let (mass_1, mass_2, restitution_1, restitution_2, e, j, v_r): (f32, f32, f32 ,f32, f32, f32, Vector3<f32>);
                // Mass for respective object
                let mass_1 = object_1.get_mass();
                let mass_2 = object_2.get_mass();
                // BOUNCINESS for respective object
                let restitution_1 = object_1.restitution;
                let restitution_2 = object_2.restitution;
                // Coefficient of resitution (e), use smallest one
                let e = restitution_1.min(restitution_2);
                // Relative velocity
                let v_r = object_1.get_velocity() - object_2.get_velocity();
                // j = magnitude of impulse used to calculate new velocities
                let j = -(1. + e) * (v_r.dot(&manifold.normal)) / (1./mass_1 + 1./mass_2);
                
                // Calculate new velocities: 
                object_1.set_velocity(object_1.get_velocity() - manifold.normal.scale(j / object_2.get_mass()));                
                object_2.set_velocity(object_2.get_velocity() - manifold.normal.scale(j / object_1.get_mass()));

                // FRICTION: 

                // t = tangent vector
                let t = v_r - &manifold.normal.scale(v_r.dot(&manifold.normal));
                let mut jt = -(1. + e) * (v_r.dot(&t)) / (1./mass_1 + 1./mass_2);
                jt = jt.max(-j*object_1.friction).max(-j*object_2.friction);
                jt = jt.min(j*object_1.friction).min(j*object_2.friction);
            }
        }

        // update positions
        self.update_positions(time_step);
    }

    // WILL PROBABLY UPDATE/REPLACE THIS: 
    /// Updates the positions according to their linear velocity, with timestep `time`
    fn update_positions(&mut self, time_step: f32) {
        for object in &mut self.objects {
            //object.position = Translation::from(object.get_velocity() * time_step) * object.position;
            //object.position.translation.vector + object.velocity*time_step;
            object.integrate();
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
