use kiss3d::nalgebra::{Isometry3, Point3, Vector3};

use crate::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::PhysicsScene,
    shapes::{sphere::Sphere, GameObject},
};

#[test]
fn kiss3d_rendering() {
    let mut renderer = Kiss3dRenderer::new("Test_rendering", 1000, 600);
    let mut scene = PhysicsScene::new();

    let sphere_1 = Box::new(Sphere::new(0.4));
    let transform_1 = Isometry3::translation(0.0, 0.0, 1.5);
    let sphere_2 = Box::new(Sphere::new(0.2));
    let transform_2 = Isometry3::translation(1.0, 0.0, 1.0);
    let object_1 = GameObject {
        shape: sphere_1,
        position: transform_1,
        velocity: Vector3::new(0.01, 0.0, 0.0),
        mass: 0.0,
    };

    renderer.add_obj(&object_1).unwrap();

    let object_2 = GameObject {
        shape: sphere_2,
        position: transform_2,
        velocity: Vector3::new(0.0, 0.0, 0.0),
        mass: 0.0,
    };

    renderer.add_obj(&object_2).unwrap();
    scene.add(object_1);
    scene.add(object_2);

    renderer.change_camera_speed(0.01);
    renderer.set_background(0.01, 0.01, 0.01);
    renderer.set_point_light_source(Point3::new(0.0, 5.0, 2.0));
    loop {
        scene.update(0.1);
        renderer.draw(&scene.objects()).unwrap();
    }
}
