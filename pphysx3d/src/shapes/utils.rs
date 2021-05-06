use kiss3d::nalgebra::{Isometry3, Vector3};

pub trait IsometryOperations {
    fn global_vector(&self, vector: &Vector3<f32>) -> Vector3<f32>;
}

impl IsometryOperations for &Isometry3<f32> {
    //Change vector from local space to global space
    fn global_vector(&self, vector: &Vector3<f32>) -> Vector3<f32> {
        self.rotation.to_rotation_matrix().into_inner() * vector //Via basis matrix of the rotation
    }
}
