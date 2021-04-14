use kiss3d::nalgebra::{Point, Point3};
use crate::shapes::sphere::Sphere;
use super::*;


#[test]
fn broad_phase_collision() {
    let sphere_1 = Box::new(Sphere::new(1.0f32, Point3::new(0f32, 0f32, 0f32)));
    let sphere_2 = Box::new(Sphere::new(1.0f32, Point3::new(1.99f32, 1.99f32, 0f32)));
    let objects = vec![sphere_1, sphere_2];
    let collisions = broad_phase(&objects);
    //println!("{:?}", collisions.len());
    assert_eq!(collisions.len(), 4)
    //print!("{:?} \n {:?}\n", objects[0].compute_aabb(), objects[1].compute_aabb())
}