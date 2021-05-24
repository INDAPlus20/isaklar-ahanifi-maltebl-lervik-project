extern crate pphysx3d;
use pphysx3d::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::{
        game_object::{GameObject, INFINITY},
        PhysicsScene,
    },
};

fn main() {
    let mut renderer = Kiss3dRenderer::new("Demo", 1200, 900);
    let mut scene = PhysicsScene::new();

    let mut sphere_1 = GameObject::Sphere_default(0.4, [0, 0, 0], [1.0, 2.0, 10.], 10., 1., 0.01);
    sphere_1.add_velocity([10.0, 0.0, 0.0]);
    renderer.add_obj(&sphere_1).unwrap();
    scene.add(sphere_1);

    let mut sphere_2 = GameObject::Sphere_default(0.4, [255, 0, 0], [0.0, 10.0, 10.], 8., 1., 0.01);
    sphere_2.add_velocity([8.0, -5.0, 0.0]);
    renderer.add_obj(&sphere_2).unwrap();
    scene.add(sphere_2);

    let mut sphere_3 =
        GameObject::Sphere_default(0.5, [255, 100, 0], [2.0, 10.0, 10.], 10., 1., 0.01);
    sphere_3.add_velocity([8.0, -5.0, 1.0]);
    renderer.add_obj(&sphere_3).unwrap();
    scene.add(sphere_3);

    let mut sphere_4 =
        GameObject::Sphere_default(0.1, [255, 0, 255], [-1.0, 10.0, 10.], 2., 1., 0.01);
    sphere_4.add_velocity([8.0, 0.0, 0.0]);
    renderer.add_obj(&sphere_4).unwrap();
    scene.add(sphere_4);

    let mut sphere_5 = GameObject::Sphere_default(0.2, [255, 40, 0], [0.0, 5.0, 5.], 6., 1., 0.01);
    sphere_5.add_velocity([3.0, -1.0, 0.0]);
    renderer.add_obj(&sphere_5).unwrap();
    scene.add(sphere_5);

    let plane_1 = GameObject::Plane_default(
        [0., 1., 0.],
        [255, 255, 255],
        [0.0, -1.0, 0.0],
        INFINITY,
        1.,
        0.1,
    );
    renderer.add_obj(&plane_1).unwrap();
    scene.add(plane_1);

    let plane_2 = GameObject::Plane_default(
        [1., 0., 0.],
        [0, 255, 255],
        [-4.0, 0.0, 0.0],
        INFINITY,
        1.,
        0.1,
    );
    renderer.add_obj(&plane_2).unwrap();
    scene.add(plane_2);

    let plane_3 = GameObject::Plane_default(
        [-1., 0., 0.],
        [0, 255, 255],
        [3.0, 0.0, 0.0],
        INFINITY,
        1.,
        0.1,
    );
    renderer.add_obj(&plane_3).unwrap();
    scene.add(plane_3);

    let plane_4 = GameObject::Plane_default(
        [0., 0., -1.],
        [0, 255, 0],
        [0.0, 0.0, 12.0],
        INFINITY,
        1.,
        0.1,
    );
    renderer.add_obj(&plane_4).unwrap();
    scene.add(plane_4);

    let plane_5 = GameObject::Plane_default(
        [0., 0., 1.],
        [0, 255, 0],
        [0.0, 0.0, -2.0],
        INFINITY,
        1.,
        0.1,
    );
    renderer.add_obj(&plane_5).unwrap();
    scene.add(plane_5);

    renderer.change_camera_speed(0.1);
    renderer.set_background(0.5, 0.5, 0.5);
    //renderer.set_light_to_camera();
    loop {
        scene.update(0.007);

        renderer.draw(&scene.objects()).unwrap();
    }
}
