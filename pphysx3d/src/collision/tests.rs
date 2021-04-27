use super::*;
use crate::shapes::sphere::Sphere;
use kiss3d::nalgebra::{Point, Point3, Rotation3, Translation3, UnitQuaternion};

#[test]
fn broad_phase_collision() {
    let sphere_1 = Box::new(Sphere::new(1.0f32, Point3::new(0f32, 0f32, 0f32)));
    let sphere_2 = Box::new(Sphere::new(1.0f32, Point3::new(1.99f32, 1.99f32, 0f32)));
    let objects = vec![sphere_1, sphere_2];
    let collisions = broad_phase(&objects);
    //println!("{:?}", collisions.len());
    assert_eq!(collisions.len(), 1)
    //print!("{:?} \n {:?}\n", objects[0].compute_aabb(), objects[1].compute_aabb())
}

#[test]
fn sphere_sphere_collision_check() {
    let sphere_1 = Sphere::new(2.0f32, Point3::new(0f32, 0f32, 0f32));
    let sphere_2 = Sphere::new(2.0f32, Point3::new(0f32, 2f32, 3f32));
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(0f32, 2f32, 2f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let result = sphere_sphere(&sphere_1, &sphere_2, &iso_1, &iso_2);
    assert_eq!(result, true)
}

#[test]
fn sphere_manifold() {
    let sphere_1 = Sphere::new(2.0f32, Point3::new(0f32, 0f32, 0f32));
    let sphere_2 = Sphere::new(2.0f32, Point3::new(0f32, 0f32, 0f32));
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(0f32, 3f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let result = CollisionManifold::sphere_sphere(&sphere_1, &sphere_2, &iso_1, &iso_2);
    let mut contacts = Vec::with_capacity(10);
    contacts.push(Point3::new(0f32, 1.5f32, 0f32));
    let facit = CollisionManifold {
        colliding: true,
        depth: 0.5f32,
        normal: UnitVector3::new_normalize(Vector3::new(0f32, 1f32, 0f32)),
        contacts: contacts,
    };
    assert_eq!(facit, result);
}
