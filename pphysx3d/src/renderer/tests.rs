use kiss3d::{
    camera::Camera,
    nalgebra::{Isometry3, Point3, Transform3, Translation3, Unit, UnitQuaternion, Vector3},
};

use crate::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::{
        game_object::{GameObject, INFINITY},
        PhysicsScene,
    },
    shapes::{cube::Cube, plane::Plane, sphere::Sphere},
};

#[test]
#[should_panic]
fn kiss3d_rendering() {
    let mut renderer = Kiss3dRenderer::new("Test_rendering", 1000, 600);
    let mut scene = PhysicsScene::new();

    let mut sphere_1 = GameObject::Sphere_default(0.4, [255, 0, 0], [0.0, 0.0, 1.5], 0.1, 0.1, 0.2);
    sphere_1.add_velocity([10.0, 1.0, 0.0]);
    renderer.add_obj(&sphere_1).unwrap();
    scene.add(sphere_1);

    let mut sphere_2 =
        GameObject::Sphere_default(0.2, [100, 200, 0], [1.0, 0.0, 1.0], 0.1, 0.1, 0.2);
    renderer.add_obj(&sphere_2).unwrap();
    scene.add(sphere_2);

    let mut cube_1 = GameObject::Cube_default(
        [0.5, 1.0, 1.0],
        [0, 0, 0],
        [-1.0, 0.0, 1.2],
        INFINITY,
        1.0,
        0.2,
    );
    renderer.add_obj(&cube_1).unwrap();
    scene.add(cube_1);

    let mut plane_1 = GameObject::Plane_default(
        [0.0, 1.0, 0.0],
        [255, 255, 255],
        [0.0, -2.0, 0.0],
        INFINITY,
        1.0,
        0.2,
    );
    renderer.add_obj(&plane_1).unwrap();
    scene.add(plane_1);

    renderer.change_camera_speed(0.01);
    renderer.set_background(0.5, 0.5, 0.5);
    renderer.set_point_light_source(Point3::new(0.0, 5.0, 2.0));
    loop {
        scene.update(0.01);
        renderer.draw(&scene.objects()).unwrap();
    }
}

#[test]
#[should_panic]
fn sphere_fall_into_plane() {
    let mut renderer = Kiss3dRenderer::new("Test_rendering", 1000, 600);
    let mut scene = PhysicsScene::new();

    let sphere_1 = GameObject::Sphere_default(10.0, [0, 0, 0], [1.0, 2.0, 20.], 10., 1.0, 0.01);
    renderer.add_obj(&sphere_1).unwrap();
    scene.add(sphere_1);

    let mut sphere_2 =
        GameObject::Sphere_default(10.0, [255, 0, 0], [1.0, 25.0, 20.], 10., 1.0, 0.01);
    sphere_2.add_velocity([0.0, -10.0, 0.0]);
    renderer.add_obj(&sphere_2).unwrap();
    scene.add(sphere_2);

    let plane_1 = GameObject::Plane_default(
        [0., 1., 0.],
        [255, 255, 255],
        [0.0, -10.0, 0.0],
        INFINITY,
        1.,
        0.1,
    );

    renderer.add_obj(&plane_1).unwrap();
    scene.add(plane_1);
    //TODO ADD CAMERA POSITION FUNCTION;
    renderer.change_camera_speed(0.01);
    renderer.set_background(0.5, 0.5, 0.5);
    renderer.set_point_light_source(Point3::new(0.0, 5.0, 2.0));
    loop {
        scene.update(0.007);

        renderer.draw(&scene.objects()).unwrap();
    }
}
