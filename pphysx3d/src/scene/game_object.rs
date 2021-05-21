use kiss3d::nalgebra::{Isometry3, Matrix3, Translation, UnitQuaternion, Vector3};

use crate::shapes::shape::Shape;

pub const INFINITY: f32 = f32::INFINITY;
pub const DAMPING: f32 = 0.001;

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
    pub torque_accum: Vector3<f32>,     // Torque summed, same principle as force_accum [Nm]
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
            // How should mass = 0 be handled?
            // - It shouldn't be a thing
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

    pub fn add_rotational_impulse(&mut self, contact_point: Vector3<f32>, impulse: Vector3<f32>) {
        let center_of_mass = &self.position.translation;
        let torque: Vector3<f32> = (contact_point - center_of_mass.vector).cross(&impulse);
        let angular_acceleration = &self.inv_tensor() * torque;
        self.angular_velocity += angular_acceleration; // NOT SURE IF THIS IS NECESSARY
    }

    // Only works for simple shapes atm. Will need to incorporate Steiner and stuff if we want inertia tensors for composite bodies
    // May need to be slightly different from real world inerta tensors to feel realistic in a physics engine
    pub fn inv_tensor(&self) -> Matrix3<f32> {
        // An object's inertia tensor is defined by its geometric properties
        let mut inv_tensor: Matrix3<f32> = Matrix3::<f32>::zeros();
        let inv_mass = &self.inverse_mass;

        if let Ok(_sphere) = &self.shape.as_sphere() {
            let radius = &self.shape.as_sphere().unwrap().radius;
            // RADIUS CAN'T BE 0
            let diagonal: Vector3<f32> = Vector3::new(
                2.5 * inv_mass / (radius * radius),
                2.5 * inv_mass / (radius * radius),
                2.5 * inv_mass / (radius * radius),
            );
            inv_tensor.set_diagonal(&diagonal);
        }

        if let Ok(_cube) = &self.shape.as_cube() {
            let half_extents = &self.shape.as_cube().unwrap().half_extents;
            // HALF EXTENTS CAN'T BE 0
            let diagonal: Vector3<f32> = Vector3::new(
                12. * inv_mass
                    / (half_extents.y * half_extents.y + half_extents.z * half_extents.z),
                12. * inv_mass
                    / (half_extents.x * half_extents.x + half_extents.z * half_extents.z),
                12. * inv_mass
                    / (half_extents.x * half_extents.x + half_extents.y * half_extents.y),
            );
            inv_tensor.set_diagonal(&diagonal);
        }

        return inv_tensor;
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
        self.torque_accum = Vector3::new(0., 0., 0.);
    }

    // Pretty much just Explicit Euler, might want to change to something like Verlet
    pub fn integrate(&mut self, dt: f32) {
        // Update linear position
        //self.position.translation = self.position.translation.one() * Translation::from(DURATION * self.velocity);
        self.position.translation.vector = self.position.translation.vector + dt * self.velocity;

        // I'm so confused over how these work, maybe this is completely wrong:
        self.position.rotation =
            self.position.rotation * UnitQuaternion::new(0.5 * dt * self.angular_velocity);

        // Calculate acceleration from force
        self.acceleration += self.inverse_mass * self.force_accum;
        self.angular_acceleration += self.inv_tensor() * self.torque_accum;

        // Calculate new velocity
        self.velocity = (1. - DAMPING) * self.velocity + dt * self.acceleration;
        self.angular_velocity = (1. - DAMPING) * self.angular_velocity + dt * self.angular_acceleration;

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