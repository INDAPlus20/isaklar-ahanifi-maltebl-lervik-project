use kiss3d::nalgebra::{Point3, Translation3, UnitVector3, Vector3};

use crate::{
    collision::CollisionManifold,
    scene::{broad_phase, game_object::GameObject, narrow_phase, PhysicsScene},
    shapes::{bounding_volume::BoundingVolume, sphere::Sphere},
};

#[test]
fn update_position_test() {
    let mut sphere1 = GameObject::Sphere_default(2., [100, 200, 0], [0.; 3], 10., 0.1, 0.2);
    sphere1.add_velocity([1., 0., 0.]);

    let mut sphere2 = GameObject::Sphere_default(2., [100, 200, 0], [0.; 3], 10., 0.1, 0.2);
    sphere2.add_velocity([0., 0., 1.]);

    let objects = vec![sphere1, sphere2];
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
    let sphere1 = GameObject::Sphere_default(1., [100, 200, 0], [0.; 3], 10., 0.1, 0.2);
    let sphere2 = GameObject::Sphere_default(1., [100, 200, 0], [1.99, 1.99, 0.], 10., 0.1, 0.2);

    let objects = vec![sphere1, sphere2];
    let collisions = broad_phase(&objects);
    assert_eq!(collisions.len(), 1);
    assert_eq!(0, collisions[0].0);
    assert_eq!(1, collisions[0].1);
}

#[test]
fn narrow_phase_collision() {
    let mut sphere1 = GameObject::Sphere_default(2., [100, 200, 0], [0.; 3], 10., 0.1, 0.2);
    sphere1.add_velocity([10., 0., 0.]);

    let mut sphere2 = GameObject::Sphere_default(2., [100, 200, 0], [3., 0., 0.], 10., 0.1, 0.2);
    sphere2.add_velocity([0., 0., -10.]);

    let objects = vec![sphere1, sphere2];
    let collisions = broad_phase(&objects);

    let manifolds = narrow_phase(&objects, &collisions);
    let check = CollisionManifold {
        colliding: true,
        depth: 0.5f32,
        normal: UnitVector3::new_normalize(Vector3::new(1f32, 0f32, 0f32)),
        contacts: vec![Point3::new(1.5f32, 0f32, 0f32)],
    };
    assert_eq!(manifolds[0], check)
}
