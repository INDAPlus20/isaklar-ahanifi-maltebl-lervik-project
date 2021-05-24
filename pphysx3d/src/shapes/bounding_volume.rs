use kiss3d::nalgebra::{self as na, Point3, Vector3};

pub trait BoundingVolume {
    fn interects(&self, other: &Self) -> bool;
    fn contains(&self, other: &Self) -> bool;
}

pub struct AABB {
    //two points is all it takes to represent a box
    mins: Point3<f32>,
    maxs: Point3<f32>,
}

impl AABB {
    /// Creates a new axis-aligned bounding box with the given min points and max points
    pub fn new(mins: Point3<f32>, maxs: Point3<f32>) -> AABB {
        AABB { mins, maxs }
    }
    pub fn mins(&self) -> Point3<f32> {
        self.mins
    }
    pub fn maxs(&self) -> Point3<f32> {
        self.maxs
    }
    /// Creates a new axis-aligned bounding box with the given half extents    
    pub fn from_half_extents(center: Point3<f32>, half_extents: Vector3<f32>) -> AABB {
        let mins: Point3<f32> = center - half_extents;
        let maxs: Point3<f32> = center + half_extents;
        AABB { mins, maxs }
    }

    /// Translates the axis-aligned bounding box with the given vector
    pub fn translate(&mut self, translation: &Vector3<f32>) {
        self.maxs += translation;
        self.mins += translation;
    }
}

impl BoundingVolume for AABB {
    fn contains(&self, other: &Self) -> bool {
        na::partial_le(&self.mins, &other.mins()) && na::partial_ge(&self.maxs, &other.maxs())
    }
    /// Checks if there is an intersection. Also returns true if self contains other
    fn interects(&self, other: &Self) -> bool {
        //a::partial_le(&self.mins, &other.maxs) && na::partial_ge(&self.maxs, &other.mins)
        self.mins <= other.maxs() && self.maxs >= other.mins()
    }
}

pub struct BoundingSphere {
    center: Point3<f32>,
    radius: f32,
}

impl BoundingSphere {
    /// Creates a new bounding sphere from a radius and center
    pub fn new(radius: f32, center: Point3<f32>) -> BoundingSphere {
        BoundingSphere { center, radius }
    }
}

impl BoundingVolume for BoundingSphere {
    fn contains(&self, other: &Self) -> bool {
        let difference: Vector3<f32> = other.center - self.center;
        difference.norm() + other.radius <= self.radius
    }
    fn interects(&self, other: &Self) -> bool {
        let difference: Vector3<f32> = other.center - self.center;
        let distance_squared = difference.norm_squared();
        let radius_summed = self.radius + other.radius;

        distance_squared <= radius_summed * radius_summed
    }
}
