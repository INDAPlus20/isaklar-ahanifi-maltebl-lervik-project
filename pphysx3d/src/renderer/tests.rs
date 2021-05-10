use kiss3d::{
    camera::Camera,
    nalgebra::{Isometry3, Point3, Unit, UnitQuaternion, Vector3},
};

use crate::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::PhysicsScene,
    shapes::{cube::Cube, plane::Plane, sphere::Sphere, GameObject},
};

#[test]
#[should_panic]
fn kiss3d_rendering() {
    let mut renderer = Kiss3dRenderer::new("Test_rendering", 1000, 600);
    let mut scene = PhysicsScene::new();

    let sphere_1 = Box::new(Sphere::new(0.4));
    let transform_1 = Isometry3::translation(0.0, 0.0, 1.5);
    let object_1 = GameObject {
        shape: sphere_1,
        position: transform_1,
        velocity: Vector3::new(0.01, 0.0, 0.0),
        acceleration: Vector3::new(0., 0., 0.),
        force_accum: Vector3::new(0., 0., 0.),
        inverse_mass: 0.1,
        bounciness: 0.1,
        friction: 0.2,
    };
    renderer.add_obj(&object_1).unwrap();
    scene.add(object_1);

    let sphere_2 = Box::new(Sphere::new(0.2));
    let transform_2 = Isometry3::translation(1.0, 0.0, 1.0);
    let object_2 = GameObject {
        shape: sphere_2,
        position: transform_2,
        velocity: Vector3::new(0.0, 0.0, 0.0),
        acceleration: Vector3::new(0., 0., 0.),
        force_accum: Vector3::new(0., 0., 0.),
        inverse_mass: 0.1,
        bounciness: 0.0,
        friction: 0.0,
    };
    renderer.add_obj(&object_2).unwrap();
    scene.add(object_2);

    let cube_1 = Box::new(Cube::new(Vector3::new(0.5, 1.0, 1.0)));
    let transform_3 = Isometry3::translation(-1.0, 0.0, 1.2);
    let object_3 = GameObject {
        shape: cube_1,
        position: transform_3,
        velocity: Vector3::new(0.0, 0.0, 0.0),
        acceleration: Vector3::new(0., 0., 0.),
        force_accum: Vector3::new(0., 0., 0.),
        inverse_mass: 0.1,
        bounciness: 0.0,
        friction: 0.0,
    };
    renderer.add_obj(&object_3).unwrap();
    scene.add(object_3);

    let plane_1 = Box::new(Plane::new(Unit::new_normalize(Vector3::new(0., 1., 0.))));
    let transform_4 = Isometry3::translation(0.0, -1.0, 0.0);
    let object_4 = GameObject {
        shape: plane_1,
        position: transform_4,
        velocity: Vector3::new(0.0, 0.0, 0.0),
        acceleration: Vector3::new(0., 0., 0.),
        force_accum: Vector3::new(0., 0., 0.),
        inverse_mass: 0.,
        bounciness: 0.0,
        friction: 0.0,
    };
    renderer.add_obj(&object_4).unwrap();
    scene.add(object_4);

    renderer.change_camera_speed(0.01);
    renderer.set_background(0.5, 0.5, 0.5);
    renderer.set_point_light_source(Point3::new(0.0, 5.0, 2.0));
    loop {
        scene.update(0.1);
        renderer.draw(&scene.objects()).unwrap();
    }
}