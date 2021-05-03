use super::*;
use crate::shapes::sphere::Sphere;
use kiss3d::nalgebra::{Point, Point3, Rotation3, Translation3, UnitQuaternion};

#[test]
fn broad_phase_collision() {
    let sphere_1 = Box::new(Sphere::new(1.0f32));
    let transform_1 = Isometry3::translation(0f32, 0f32, 0f32);
    let sphere_2 = Box::new(Sphere::new(1.0f32));
    let transform_2 = Isometry3::translation(1.99f32, 1.99f32, 0f32);
    let object_1 = GameObject {
        shape: sphere_1,
        position: transform_1,
        velocity: Vector3::new(0f32, 0f32, 0f32),
        mass: 0f32,
    };
    let object_2 = GameObject {
        shape: sphere_2,
        position: transform_2,
        velocity: Vector3::new(0f32, 0f32, 0f32),
        mass: 0f32,
    };
    let objects = vec![object_1, object_2];
    let collisions = broad_phase(&objects);
    //println!("{:?}", collisions.len());
    assert_eq!(collisions.len(), 1)
    //print!("{:?} \n {:?}\n", objects[0].compute_aabb(), objects[1].compute_aabb())
}

#[test]
fn sphere_sphere_collision_check() {
    let sphere_1 = Sphere::new(2.0f32);
    let sphere_2 = Sphere::new(2.0f32);
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
    let sphere_1 = Sphere::new(2.0f32);
    let sphere_2 = Sphere::new(2.0f32);
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

#[test]
fn narrow_phase_collision() {
    let sphere_1 = Box::new(Sphere::new(2.0f32));
    let sphere_2 = Box::new(Sphere::new(2.0f32));
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(0f32, 3f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let object_1 = GameObject {
        shape: sphere_1,
        position: iso_1,
        velocity: Vector3::new(0f32, 0f32, 0f32),
        mass: 0f32,
    };
    let object_2 = GameObject {
        shape: sphere_2,
        position: iso_2,
        velocity: Vector3::new(0f32, 0f32, 0f32),
        mass: 0f32,
    };
    let objects = vec![object_1, object_2];
    let collisions = broad_phase(&objects);

    let manifolds = narrow_phase(&collisions);
    let check = CollisionManifold {
        colliding: true,
        depth: 0.5f32,
        normal: UnitVector3::new_normalize(Vector3::new(0f32, 1f32, 0f32)),
        contacts: vec![Point3::new(0f32, 1.5f32, 0f32)],
    };
    assert_eq!(manifolds[0], check)
}
