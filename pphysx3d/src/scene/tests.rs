use super::*;
use kiss3d::nalgebra::Translation3;
use kiss3d::nalgebra::{Isometry3, Point3, Unit, UnitQuaternion, UnitVector3, Vector3};

use crate::shapes::{bounding_volume::BoundingVolume, sphere::Sphere};

#[test]
fn update_position_test() {
    let sphere_1 = Box::new(Sphere::new(2.0f32));
    let sphere_2 = Box::new(Sphere::new(2.0f32));
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let object_1 = GameObject::new(
        sphere_1,
        [100, 200, 0],
        iso_1,
        [1.0, 0.0, 0.0],
        10.,
        0.1,
        0.2,
    );
    let object_2 = GameObject::new(
        sphere_2,
        [100, 200, 0],
        iso_2,
        [0.0, 0.0, 1.0],
        10.,
        0.1,
        0.2,
    );
    let objects = vec![object_1, object_2];
    let mut scene = PhysicsScene::new();
    for object in objects {
        scene.add(object)
    }

    scene.update_positions(1.);
    assert_eq!(
        scene.objects[0].position.translation,
        Translation3::new(1f32, 0f32, 0f32)
    );
    assert_eq!(
        scene.objects[1].position.translation,
        Translation3::new(0f32, 0f32, 1f32)
    );
}

#[test]
fn broad_phase_collision() {
    let sphere_1 = Box::new(Sphere::new(1.0f32));
    let transform_1 = Isometry3::translation(0f32, 0f32, 0f32);
    let sphere_2 = Box::new(Sphere::new(1.0f32));
    let transform_2 = Isometry3::translation(1.99f32, 1.99f32, 0f32);
    let object_1 = GameObject::new(
        sphere_1,
        [100, 200, 0],
        transform_1,
        [0.0, 0.0, 0.0],
        10.,
        0.1,
        0.2,
    );
    let object_2 = GameObject::new(
        sphere_2,
        [100, 200, 0],
        transform_2,
        [0.0, 0.0, 0.0],
        10.,
        0.1,
        0.2,
    );
    let objects = vec![object_1, object_2];
    let collisions = broad_phase(&objects);
    assert_eq!(collisions.len(), 1);
    assert_eq!(0, collisions[0].0);
    assert_eq!(1, collisions[0].1);
}

#[test]
fn narrow_phase_collision() {
    let sphere_1 = Box::new(Sphere::new(2f32));
    let sphere_2 = Box::new(Sphere::new(2f32));
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(3f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let object_1 = GameObject::new(
        sphere_1,
        [100, 200, 0],
        iso_1,
        [10.0, 0.0, 0.0],
        10.,
        0.1,
        0.2,
    );
    let object_2 = GameObject::new(
        sphere_2,
        [200, 100, 0],
        iso_2,
        [-10.0, 0.0, 0.0],
        10.,
        0.1,
        0.2,
    );

    let objects = vec![object_1, object_2];
    let collisions = broad_phase(&objects);

    let manifolds = narrow_phase(&objects, &collisions);
    let check = CollisionManifold {
        colliding: true,
        depth: 1f32,
        normal: UnitVector3::new_normalize(Vector3::new(1f32, 0f32, 0f32)),
        contacts: vec![Point3::new(0f32, 0f32, 0f32)],
    };
    assert_eq!(manifolds[0], check)
}
