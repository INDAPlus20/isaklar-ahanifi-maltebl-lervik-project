use kiss3d::nalgebra::{self as na, Isometry3, Point3, Translation};
use na::Vector3;

use self::shape::Shape;

pub mod bounding_volume;
pub mod cube;
pub mod shape;
pub mod sphere;

mod tests;
mod utils;

pub const INFINITY: f32 = f32::INFINITY;

struct Particle {
    position: Point3<f32>,
    velocity: Vector3<f32>,
}

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

pub struct GameObject {
    pub shape: Box<dyn Shape>,    // The collider
    pub position: Isometry3<f32>, // includes a translation vector and a rotation part as an unit quaternion
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>,
    force_accum: Vector3<f32>,
    //texture:
    pub inverse_mass: f32,
    pub restitution: f32, // elasticity aka bounciness. rename to bounciness?
    pub friction: f32,    // coefficient of friction
}

impl GameObject {
    pub fn set_velocity(&mut self, velocity: Vector3<f32>) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&self) -> Vector3<f32> {
        return self.velocity;
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.inverse_mass = 1. / mass;
    }

    pub fn set_inverse_mass(&mut self, inverse_mass: f32) {
        self.inverse_mass = inverse_mass;
    }

    pub fn get_mass(&self) -> f32 {
        if self.inverse_mass != 0. {
            return 1. / self.inverse_mass;
        }
        return INFINITY;
    }

    pub fn add_force(&mut self, force: Vector3<f32>) {
        self.force_accum = self.force_accum + force;
    }

    fn clear_accum(&mut self) {
        // maybe don't have to create a new Vector3 idk yet
        self.force_accum = Vector3::new(0., 0., 0.);
    }

    // Pretty much just Explicit Euler, might want to change to something like Verlet
    pub fn integrate(&mut self, dt: f32) {
        // Update linear position
        //self.position.translation = self.position.translation.one() * Translation::from(DURATION * self.velocity);
        self.position.translation =
            Translation::from(self.position.translation.vector + dt * self.velocity);

        // Calculate acceleration from force
        self.acceleration += self.inverse_mass * self.force_accum;

        // Calculate new velocity
        self.velocity += dt * self.acceleration;

        // (NOT SURE IF HAVE TO MAKE NEW ZERO VECTOR)
        self.clear_accum();
    }
}
