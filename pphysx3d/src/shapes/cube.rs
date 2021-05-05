use kiss3d::nalgebra::{Isometry3, Point3, Vector3};

use super::{
    bounding_volume::{BoundingSphere, AABB},
    shape::Shape,
    utils::IsometryOperations,
};

struct Cube {
    pub half_extents: Vector3<f32>, //x y z
}

impl Cube {
    fn new(half_extents: Vector3<f32>) -> Cube {
        Cube { half_extents }
    }
    /// Returns the axis-aligned bounding box of the cube with the position given Isome
    fn aabb(&self, pos: &Isometry3<f32>) -> AABB {
        let center = Point3::from(pos.translation.vector);
        let absolute_pos_vector: Vector3<f32> = pos.global_vector(&self.half_extents); // change of basis so we get the half extents in global space

        AABB::from_half_extents(center, absolute_pos_vector)
    }

    fn bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere {
        let radius = self.half_extents.norm();
        let center = Point3::from(pos.translation.vector);
        BoundingSphere::new(radius, center)
    }
}

impl Shape for Cube {
    fn compute_aabb(&self, pos: &Isometry3<f32>) -> AABB {
        self.aabb(pos)
    }
    fn compute_bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere {
        self.bounding_sphere(pos)
    }
}
