use kiss3d::nalgebra::{Point3, UnitVector3};

pub struct Ray {
    origin: Point3<f32>,
    direction: UnitVector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: UnitVector3<f32>) -> Ray {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> &Point3<f32> {
        return &self.origin;
    }
    pub fn direction(&self) -> &UnitVector3<f32> {
        return &self.direction;
    }
}
