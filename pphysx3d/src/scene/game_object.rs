use kiss3d::nalgebra::{Isometry3, Translation, Vector3};

use crate::shapes::shape::Shape;

pub const INFINITY: f32 = f32::INFINITY;

pub struct GameObject {
    shape: Box<dyn Shape>, // The collider
    //texture:
    color: [u8; 3], //RGB values for the object's default colour (overwritten if texture exists)
    inverse_mass: f32, // [1/kg]
    bounciness: f32, // elasticity aka coefficient of restitution
    friction: f32,  // coefficient of friction
    // Regular momentum stuff:
    pub position: Isometry3<f32>, // includes a translation vector and a rotation part as an unit quaternion
    pub velocity: Vector3<f32>,   // [m/s]
    pub acceleration: Vector3<f32>, // [m/s^2]
    force_accum: Vector3<f32>,    // Forces summed a la d'Alembert's principle [N]
    // Angular momentum stuff:
    // Orientation is stored in the unit quaternion of position
    //pub orientation: Vector3<f32>, // Object's orientation in the room, angular equivalent to position
    pub angular_velocity: Vector3<f32>, // Angular velocity [rad/s]
    pub angular_acceleration: Vector3<f32>, // Angular acceleration [rad/s^2]
    pub torque_accum: Vector3<f32>, // Torque summed, same principle as force_accum [Nm]
}

impl GameObject {
    pub fn new(
        shape: Box<dyn Shape>,
        color: [u8; 3],
        position: Isometry3<f32>,
        velocity: [f32; 3],
        mass: f32,
        bounciness: f32,
        friction: f32,
    ) -> GameObject {
        let inv_mass = if mass >= INFINITY - f32::EPSILON {
            0.
        } else if mass <= f32::EPSILON {
            //How should mass= 0 be handled?
            INFINITY
        } else {
            1. / mass
        };

        GameObject {
            shape,
            color,
            inverse_mass: inv_mass,
            bounciness,
            friction,
            position,
            velocity: Vector3::from(velocity),
            acceleration: Vector3::new(0., 0., 0.),
            force_accum: Vector3::new(0., 0., 0.),
            angular_velocity: Vector3::new(0., 0., 0.),
            angular_acceleration: Vector3::new(0., 0., 0.),
            torque_accum: Vector3::new(0., 0., 0.),
        }
    }

    ///The Object's shape
    pub fn shape(&self) -> &dyn Shape {
        self.shape.as_ref()
    }

    ///All accumulated forces acting on the Object
    pub fn force_accum(&self) -> &Vector3<f32> {
        &self.force_accum
    }

    ///Add a force acting on the object to it's force accumulator
    pub fn add_force(&mut self, force: Vector3<f32>) {
        self.force_accum = self.force_accum + force;
    }

    ///remove all accumulated forces acting on the Object
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

    pub fn color(&self) -> [u8; 3] {
        self.color
    }

    ///The mass of the Object (1/inverse_mass)
    pub fn mass(&self) -> f32 {
        if self.inverse_mass != 0. {
            return 1. / self.inverse_mass;
        }
        return INFINITY;
    }

    ///The inverse mass of the Object
    pub fn inv_mass(&self) -> f32 {
        if self.inverse_mass != 0. {
            return 1. / self.inverse_mass;
        }
        return 0.;
    }

    ///The Object's coefficient of bounciness
    pub fn bounciness(&self) -> f32 {
        self.bounciness
    }
    ///The Object's coefficient of friction
    pub fn friction(&self) -> f32 {
        self.friction
    }
}
