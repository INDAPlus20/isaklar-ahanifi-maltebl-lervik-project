use crate::collision::*;
use crate::shapes::GameObject;

pub struct PhysicsScene {
    pub objects: Vec<GameObject>,
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

    /// Updates the positions according to their linear velocity, with timestep `time`
    fn update_positions(&mut self, time_step: f32) {
        for object in &mut self.objects {
            object.position.translation.vector =
                object.position.translation.vector + object.velocity * time_step;
        }
    }
}
