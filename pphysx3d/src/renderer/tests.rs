use kiss3d::nalgebra::{Isometry3, Vector3};

use crate::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::PhysicsScene,
    shapes::{sphere::Sphere, GameObject},
};

#[test]
fn kiss3d_rendering() {
    let mut renderer = Kiss3dRenderer::new("Test_rendering", 400, 600);
    let mut scene = PhysicsScene::new();

    let sphere_1 = Box::new(Sphere::new(0.4f32));
    let transform_1 = Isometry3::translation(0f32, 0f32, 0.5f32);
    let sphere_2 = Box::new(Sphere::new(0.2f32));
    let transform_2 = Isometry3::translation(1.0f32, 0f32, 0f32);
    let object_1 = GameObject {
        shape: sphere_1,
        position: transform_1,
        velocity: Vector3::new(0.01f32, 0f32, 0f32),
        mass: 0f32,
    };

    renderer.add_obj(&object_1).unwrap();

    let object_2 = GameObject {
        shape: sphere_2,
        position: transform_2,
        velocity: Vector3::new(0f32, 0f32, 0f32),
        mass: 0f32,
    };

    renderer.add_obj(&object_2).unwrap();
    scene.add(object_1);
    scene.add(object_2);
    loop {
        scene.update(0.1);
        renderer.draw(&scene.get_objects()).unwrap();
    }
}
