use crate::collision::*;
use crate::shapes::bounding_volume::BoundingVolume;
use game_object::GameObject;
use kiss3d::nalgebra::{Translation, Unit, UnitVector3, Vector3};
use std::cmp::min;

pub mod game_object;
mod tests;

// For gravity!!!
const g: f32 = 9.82;

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
                let contacts = manifold.contacts.len() as f32;
                let inv_tensor_1 = self.objects[index.0].inv_tensor();
                let inv_tensor_2 = self.objects[index.1].inv_tensor();

                // Calculate impulse for every contact point in collision
                for (_, contact) in manifold.contacts.iter().enumerate() {
                    let manifold_normal = &manifold.normal;

                    // Relative position from center of mass to contact point for respective object
                    let relative_vector_1: Vector3<f32> =
                        contact.coords - self.objects[index.0].position.translation.vector;
                    let relative_vector_2: Vector3<f32> =
                        contact.coords - self.objects[index.1].position.translation.vector;

                    let [(impulse1, friction1), (impulse2, friction2)] = self.calculate_impulse(
                        index.0,
                        index.1,
                        manifold_normal,
                        &relative_vector_1,
                        &relative_vector_2,
                    );

                    // Possible bug: might need to check whether our tangent impulse aka friction is equal to zero, and change the formula for that case

                    // Change velocity of object_1:
                    self.objects[index.0].velocity -=
                        (manifold.normal.scale(impulse1) + friction1) / contacts;
                    self.objects[index.0].angular_velocity -= inv_tensor_1
                        * relative_vector_1.cross(&manifold.normal.scale(impulse1))
                        / contacts;

                    // Change velocity of object_2:
                    self.objects[index.1].velocity +=
                        (manifold.normal.scale(impulse2) + friction2) / contacts;
                    self.objects[index.1].angular_velocity += inv_tensor_2
                        * relative_vector_2.cross(&manifold.normal.scale(impulse2))
                        / contacts;
                }
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
        r_1: &Vector3<f32>, // relative position vectors
        r_2: &Vector3<f32>,
        //manifold: &CollisionManifold,
    ) -> [(f32 /* impulse */, Vector3<f32> /*friction */); 2] {
        let object_1 = &self.objects[index_1];
        let object_2 = &self.objects[index_2];
        // Mass for respective object
        let invmass_1 = object_1.inv_mass();
        let invmass_2 = object_2.inv_mass();
        // Inverse inertia tensor for respective object
        let inv_tensor_1 = object_1.inv_tensor();
        let inv_tensor_2 = object_2.inv_tensor();
        // Relative velocity
        let v_r = (object_2.velocity + object_2.angular_velocity.cross(r_2)) // Linear + rotational velocity for object 1
            - (object_1.velocity + object_1.angular_velocity.cross(r_1)); // Linear + rotational velocity for object 2

        // COLLISION:
        // Coefficient of resitution (e), use smallest BOUNCINESS for the objects
        let e = object_1.bounciness().min(object_2.bounciness());
        // Magnitude of impulse used to calculate new velocities
        // Some of these multiplications may look like possible division by zero if inverse_mass = 0 for both objects. However, that'd mean they're both immovable which means they can't collide. Could also be added as an extra check in broad_phase just to be sure.
        let impulse_magnitude = -(1. + e) * (v_r.dot(manifold_normal))
            / (invmass_1
                + invmass_2
                + manifold_normal.dot(&(inv_tensor_1 * (r_1.cross(manifold_normal))).cross(r_1))
                + manifold_normal.dot(&(inv_tensor_2 * (r_2.cross(manifold_normal))).cross(r_2)));

        // FRICTION:
        // Tangent vector for the collision
        let tangent_vector = (v_r - manifold_normal.scale(v_r.dot(manifold_normal))).normalize();
        // Magnitude of friction
        let mut friction_magnitude = -(v_r.dot(&tangent_vector))
            / (invmass_1
                + invmass_2
                + manifold_normal.dot(&(inv_tensor_1 * (r_1.cross(&tangent_vector))).cross(r_1))
                + manifold_normal.dot(&(inv_tensor_2 * (r_2.cross(&tangent_vector))).cross(r_2)));
        let friction = (object_1.friction() * object_2.friction()).sqrt();
        friction_magnitude = friction_magnitude
            .max(-impulse_magnitude * friction)
            .min(impulse_magnitude * friction);

        [
            (
                impulse_magnitude * invmass_1,
                tangent_vector * friction_magnitude * invmass_1,
            ),
            (
                impulse_magnitude * invmass_2,
                tangent_vector * friction_magnitude * invmass_2,
            ),
        ]
    }

    /// Updates the positions according to their linear velocity, with timestep `DURATION` declared in shapes/mod.rs
    fn update_positions(&mut self, time_step: f32) {
        let gravity: Vector3<f32> = Vector3::new(0., -g, 0.); // would declare as constant Vector3 but our nalgebra is too outdated for that atm
        for object in &mut self.objects {
            // If object is immovable, aka object has infinite mass, then don't apply gravity (as it would be an infinite force)
            if object.inv_mass() > f32::EPSILON {
                object.add_force(time_step * gravity * object.mass());
            }
            // Integrate one time step
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
                    .shape()
                    .compute_aabb(&current.position)
                    .interects(&test.shape().compute_aabb(&test.position))
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
        if obj_1.velocity.x * obj_2.velocity.x > 0.0
            && obj_1.velocity.y * obj_2.velocity.y > 0.0
            && obj_1.velocity.z * obj_2.velocity.z > 0.0
        {
            manifolds.push(CollisionManifold::new());
            continue;
        }
        // if obj_1.velocity.dot(&obj_2.velocity) > 0.0 {
        //     manifolds.push(CollisionManifold::new());
        //     continue;
        // }
        // pattern-match the specific collision
        if let (Ok(sph_1), Ok(sph_2)) = (obj_1.shape().as_sphere(), obj_2.shape().as_sphere()) {
            let manifold =
                CollisionManifold::sphere_sphere(&sph_1, &sph_2, &obj_1.position, &obj_2.position);
            //println!("manifold: {:?}", manifold);
            manifolds.push(manifold);
        } else if let (Ok(plane), Ok(sphere)) =
            (obj_1.shape().as_plane(), obj_2.shape().as_sphere())
        {
            let mut manifold =
                CollisionManifold::sphere_plane(&sphere, &plane, &obj_1.position, &obj_2.position);
            manifold.normal = UnitVector3::new_normalize(manifold.normal.scale(-1.0));
            manifolds.push(manifold);
        } else if let (Ok(sphere), Ok(plane)) =
            (obj_1.shape().as_sphere(), obj_2.shape().as_plane())
        {
            let mut manifold =
                CollisionManifold::sphere_plane(&sphere, &plane, &obj_1.position, &obj_2.position);
            manifolds.push(manifold);
        }
    }
    return manifolds;
}
