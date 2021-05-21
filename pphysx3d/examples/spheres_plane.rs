extern crate pphysx3d;
use pphysx3d::{
    renderer::{self, Kiss3dRenderer, Renderer},
    scene::{
        self,
        game_object::{GameObject, INFINITY},
        PhysicsScene,
    },
};

fn main() {
    let mut renderer = Kiss3dRenderer::new("Example:Spheres_Plane", 1000, 600);
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
    renderer.change_camera_speed(0.01);
    renderer.set_background(0.5, 0.5, 0.5);
    renderer.set_point_light_source([0.0, 5.0, 2.0]);
    loop {
        scene.update(0.007);

        renderer.draw(&scene.objects()).unwrap();
    }
}