use kiss3d::nalgebra::{UnitVector3, Vector3};

pub struct Plane {
    normal: UnitVector3<f32>
}

impl Plane {
    pub fn new(normal: UnitVector3<f32>) -> Plane {
        Plane{
            normal
        }
    }

    /// Normalises the vector and constructs a new ´Plane´
    pub fn from_vector3(vector: Vector3<f32>) -> Plane {
        Plane {
            normal: UnitVector3::new_normalize(vector)
        }
    }

    /// Returns the normal of this plane
    pub fn normal(&self) -> &UnitVector3<f32> {
        return &self.normal
    }

}