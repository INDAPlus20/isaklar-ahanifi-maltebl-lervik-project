extern crate pphysx3d;
use pphysx3d::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::{
        game_object::{GameObject, INFINITY},
        PhysicsScene,
    },
};

fn main() {
    let mut renderer = Kiss3dRenderer::new("Test_rendering", 1000, 600);
    let mut scene = PhysicsScene::new();

    let sphere_1 = GameObject::Sphere_default(0.4, [0, 0, 0], [1.0, 2.0, 10.], 10., 1.0, 0.01);
    renderer.add_obj(&sphere_1).unwrap();
    scene.add(sphere_1);

    let mut sphere_2 =
        GameObject::Sphere_default(0.4, [255, 0, 0], [0.0, 10.0, 10.], 10., 1.0, 0.01);
    sphere_2.add_velocity([1.0, -10.0, 0.0]);
    renderer.add_obj(&sphere_2).unwrap();
    scene.add(sphere_2);

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
    renderer.change_camera_speed(0.01);
    renderer.set_background(0.5, 0.5, 0.5);
    renderer.set_light_to_camera();
    loop {
        scene.update(0.007);

        renderer.draw(&scene.objects()).unwrap();
    }
}
