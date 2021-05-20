use kiss3d::nalgebra::{Isometry3, Point3, UnitVector3, Vector3};

use super::{
    bounding_volume::{BoundingSphere, AABB},
    ray::Ray,
    raycast::{RayCast, RayCastResult},
    shape::Shape,
    sphere::Sphere,
    utils::IsometryOperations,
};

pub struct Cube {
    /// The values of the half extents in the following order x, y, z
    pub half_extents: Vector3<f32>, //x y z
}

impl Cube {
    /// Creates a new Cube from the given ```Vector3<f32>``` of half extents where the values represent x y z respectively
    pub fn new(half_extents: Vector3<f32>) -> Cube {
        Cube { half_extents }
    }
    /// Returns the axis-aligned bounding box of the cube with the position and rotation given by the  ```pos: &Isometry3<f32>```
    pub fn aabb(&self, pos: &Isometry3<f32>) -> AABB {
        let center = Point3::from(pos.translation.vector);
        let absolute_pos_vector: Vector3<f32> = pos.global_vector(&self.half_extents); // change of basis so we get the half extents in global space

        AABB::from_half_extents(center, absolute_pos_vector)
    }

    /// Returns the bounding sphere of the cube with the position given by the ```pos: &Isometry3<f32>```
    pub fn bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere {
        let radius = self.half_extents.norm(); //the half extents can be seen as the corners of the cube and are the points furthest from the center
        let center = Point3::from(pos.translation.vector);
        BoundingSphere::new(radius, center)
    }
}

impl Shape for Cube {
    /// Returns the axis-aligned bounding box of the cube with the position and rotation given by the  ```pos: &Isometry3<f32>```
    fn compute_aabb(&self, pos: &Isometry3<f32>) -> AABB {
        self.aabb(pos)
    }
    /// Returns the bounding sphere of the cube with the position given by the ```pos: &Isometry3<f32>```
    fn compute_bounding_sphere(&self, pos: &Isometry3<f32>) -> BoundingSphere {
        self.bounding_sphere(pos)
    }
    fn as_sphere(&self) -> Result<&Sphere, ()> {
        Err(())
    }

    fn as_cube(&self) -> Result<&Cube, ()> {
        Ok(&self)
    }

    fn as_plane(&self) -> Result<&super::plane::Plane, ()> {
        Err(())
    }
}

impl RayCast for Cube {
    fn ray_cast(&self, pos: &Isometry3<f32>, ray: &Ray) -> RayCastResult {
        // Based on https://learning.oreilly.com/library/view/game-physics-cookbook/9781787123663/ch10s04.html

        let mut result = RayCastResult::new();

        let size = self.half_extents;

        let x_rotated: UnitVector3<f32> = UnitVector3::new_normalize(pos.rotation * Vector3::x());
        let y_rotated: UnitVector3<f32> = UnitVector3::new_normalize(pos.rotation * Vector3::y());
        let z_rotated: UnitVector3<f32> = UnitVector3::new_normalize(pos.rotation * Vector3::z());

        let distance_to_center: Vector3<f32> = Point3::from(pos.translation.vector) - ray.origin();

        let mut direction_proj = [
            x_rotated.dot(ray.direction()),
            y_rotated.dot(ray.direction()),
            z_rotated.dot(ray.direction()),
        ];

        let distance_proj = [
            x_rotated.dot(&distance_to_center),
            y_rotated.dot(&distance_to_center),
            z_rotated.dot(&distance_to_center),
        ];

        // stores the min/max intersection points for all three axes
        let mut t = [0f32; 6];

        for i in 0..3 {
            // if ray direction is parallel to axis
            if direction_proj[i] == 0.0 {
                // if ray origin is not inside axis-slab
                if -distance_proj[i] - size[i] > 0.0 || -distance_proj[i] + size[i] < 0.0 {
                    return result; // ray must hit all three slabs
                }
                direction_proj[i] = 0.00001 // Almost zero to avoid div-by-zero
            }
            t[i * 2 + 0] = (distance_proj[i] + size[i]) / direction_proj[i]; // min
            t[i * 2 + 1] = (distance_proj[i] - size[i]) / direction_proj[i]; // max
        }
        // entry and exit points
        let tmin = t[0].min(t[1]).max(t[2].min(t[3])).max(t[4].min(t[4]));
        let tmax = t[0].max(t[1]).min(t[2].max(t[3])).min(t[4].max(t[4]));

        // if cube is "behind" ray
        if tmax < 0.0 {
            return result;
        }
        // if not intersecting
        if tmin > tmax {
            return result;
        }

        result.distance = tmin;

        // if ray starts inside cube
        if tmin < 0.0 {
            result.distance = tmax
        }

        result.contact_point = ray.origin() + ray.direction().scale(result.distance);

        // match which normal was hit
        let normals: [UnitVector3<f32>; 6] = [
            x_rotated, -x_rotated, y_rotated, -y_rotated, z_rotated, -z_rotated,
        ];
        for i in 0..6 {
            if t[i] == result.distance {
                result.normal = normals[i];
            }
        }

        result.hit = true;

        return result;
    }
}
