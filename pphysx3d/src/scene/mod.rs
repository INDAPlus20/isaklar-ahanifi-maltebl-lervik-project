use crate::collision::*;
use crate::shapes::{bounding_volume::BoundingVolume, GameObject};
use kiss3d::nalgebra::{Translation, Vector3};
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
                // I DON'T KNOW IF THIS IS HOW RUST IS SUPPOSED TO BE WRITTEN BTW BUT CARGO IS HAPPY
                let index = &collision_pairs[i];
                // Calculate variables for readability: (unnecessary?)
                let (
                    mass_1,
                    mass_2,
                    bounciness_1,
                    bounciness_2,
                    e,
                    j,
                    t,
                    mut jt,
                    friction,
                    velocity_1,
                    velocity_2,
                    v_r,
                ): (
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    Vector3<f32>,
                    f32,
                    f32,
                    Vector3<f32>,
                    Vector3<f32>,
                    Vector3<f32>,
                );
                {
                    let object_1: &GameObject = &self.objects[index.0];
                    let object_2: &GameObject = &self.objects[index.1];
                    // Mass for respective object
                    mass_1 = object_1.get_mass();
                    mass_2 = object_2.get_mass();
                    // Relative velocity
                    velocity_1 = object_1.velocity;
                    velocity_2 = object_2.velocity;
                    v_r = velocity_1 - velocity_2;
                    // COLLISION:
                    // BOUNCINESS for respective object
                    bounciness_1 = object_1.bounciness;
                    bounciness_2 = object_2.bounciness;
                    // Coefficient of resitution (e), use smallest one
                    e = bounciness_1.min(bounciness_2);
                    // j = magnitude of impulse used to calculate new velocities
                    j = -(1. + e) * (v_r.dot(&manifold.normal))
                        / (object_1.inverse_mass + object_2.inverse_mass);
                    // FRICTION:
                    // t = tangent vector
                    t = v_r - &manifold.normal.scale(v_r.dot(&manifold.normal));
                    // jt = magnitude of friction
                    jt =
                        -(1. + e) * (v_r.dot(&t)) / (object_1.inverse_mass + object_2.inverse_mass);
                    friction = (object_1.friction * object_2.friction).sqrt();
                    jt = jt.max(-j * friction).min(j * friction);
                }

                {
                    // Change velocity of object_1:
                    let object_1: &mut GameObject = &mut self.objects[index.0];
                    object_1.velocity = velocity_1
                        - manifold.normal.scale(j / mass_2)
                        - t * jt * object_1.inverse_mass;
                }

                {
                    // Change velocity of object_2:
                    let object_2: &mut GameObject = &mut self.objects[index.1];
                    object_2.velocity = velocity_2 - manifold.normal.scale(j / mass_1)
                        + t * jt * object_2.inverse_mass;
                }
            }
        }

        // update positions
        self.update_positions(time_step);
    }

    /// Updates the positions according to their linear velocity, with timestep `DURATION` declared in shapes/mod.rs
    fn update_positions(&mut self, time_step: f32) {
        for object in &mut self.objects {
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
        if let (Some(sph_1), Some(sph_2)) = (obj_1.shape.as_sphere(), obj_2.shape.as_sphere()) {
            let manifold =
                CollisionManifold::sphere_sphere(&sph_1, &sph_2, &obj_1.position, &obj_2.position);
            manifolds.push(manifold);
        }
    }
    return manifolds;
}
