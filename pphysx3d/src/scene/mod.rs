use crate::collision::*;
use crate::shapes::{bounding_volume::BoundingVolume, GameObject};
use kiss3d::nalgebra::{Translation, Unit, Vector3};
use std::cmp::min;

mod tests;
pub struct PhysicsScene {
    objects: Vec<GameObject>,
}

impl PhysicsScene {
    pub fn new() -> PhysicsScene {
        PhysicsScene { objects: vec![] }
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

        // Resolve collisions & apply impulse + friction
        for (i, manifold) in manifolds.iter().enumerate() {
            if manifold.colliding {
                //&self.objects[i].add_force(Vector3::new(0., 0., 0.)); // this would be how to add new forces but we don't do that right here
                let index = &collision_pairs[i];
                let manifold_normal = &manifold.normal;
                let [(impulse1, friction1), (impulse2, friction2)] =
                    self.calculate_impulse(index.0, index.1, manifold_normal);

                // Change velocity of object_1:
                self.objects[index.0].velocity -= manifold.normal.scale(impulse1) + friction1;

                // Change velocity of object_2:
                self.objects[index.1].velocity -= manifold.normal.scale(impulse2) + friction2;
            }
        }

        // update positions
        self.update_positions(time_step);
    }

    fn calculate_impulse(
        &self,
        index_1: usize,
        index_2: usize,
        manifold_normal: &Unit<Vector3<f32>>,
    ) -> [(f32, Vector3<f32>); 2] {
        let object_1 = &self.objects[index_1];
        let object_2 = &self.objects[index_2];
        // Mass for respective object
        let invmass_1 = object_1.inverse_mass;
        let invmass_2 = object_2.inverse_mass;
        // Relative velocity
        let v_r = object_1.velocity - object_2.velocity;
        // COLLISION:
        // Coefficient of resitution (e), use smallest BOUNCINESS for the objects
        let e = object_1.bounciness.min(object_2.bounciness);
        // Magnitude of impulse used to calculate new velocities
        // This may look like possible division by zero if inverse_mass = 0 for both objects. However, that'd mean they're both immovable which means they can't collide. Could also be added as an extra check in broad_phase just to be sure.
        let impulse_magnitude = -(1. + e) * (v_r.dot(manifold_normal)) / (invmass_1 + invmass_2);
        // FRICTION:
        // Tangent vector for the collision
        let tangent_vector = v_r - manifold_normal.scale(v_r.dot(manifold_normal));
        // Magnitude of friction
        let mut friction_magnitude =
            -(1. + e) * (v_r.dot(&tangent_vector)) / (invmass_1 + invmass_2);
        let friction = (object_1.friction * object_2.friction).sqrt();
        friction_magnitude = friction_magnitude
            .max(-impulse_magnitude * friction)
            .min(impulse_magnitude * friction);

        [
            (
                impulse_magnitude * invmass_2,
                tangent_vector * friction_magnitude * invmass_1,
            ),
            (
                impulse_magnitude * invmass_1,
                tangent_vector * friction_magnitude * invmass_2,
            ),
        ]
    }

    /// Updates the positions according to their linear velocity, with timestep `DURATION` declared in shapes/mod.rs
    fn update_positions(&mut self, time_step: f32) {
        let gravity: Vector3<f32> = Vector3::new(0., -10., 0.); // g = 10 for now
        for object in &mut self.objects {
            if object.inverse_mass != 0. {
                object.add_force(gravity * object.mass());
            }
            object.integrate(time_step);
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
        if let (Ok(sph_1), Ok(sph_2)) = (obj_1.shape.as_sphere(), obj_2.shape.as_sphere()) {
            let manifold =
                CollisionManifold::sphere_sphere(&sph_1, &sph_2, &obj_1.position, &obj_2.position);
            manifolds.push(manifold);
        }
    }
    return manifolds;
}
