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

    let width = 4;
    let radius = 0.15;

    for x in 0..width {
        for y in 0..width {
            for z in 0..width {
                let mut sphere = GameObject::Sphere_default(
                    radius + 0.001 * x as f32 + 0.001 * y as f32 + 0.001 * z as f32,
                    [
                        ((255 / width) * x) as u8,
                        ((255 / width) * y) as u8,
                        ((255 / width) * z) as u8,
                    ],
                    [((x - 2) as f32), (y as f32), ((z + 3) as f32)],
                    10.,
                    1.,
                    0.02,
                );
                sphere.add_velocity([x as f32 - 11.5, y as f32 - 12.3, z as f32 - 11.2]);
                renderer.add_obj(&sphere).unwrap();
                scene.add(sphere);
            }
        }
    }

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
        [5.0, 0.0, 0.0],
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
    renderer.camera_position([0., 10., 0.]);
    loop {
        scene.update(0.007);

        renderer.draw(&scene.objects()).unwrap();
    }
}
