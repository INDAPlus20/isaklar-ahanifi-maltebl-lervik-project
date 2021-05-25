extern crate pphysx3d;
use pphysx3d::{
    renderer::{Kiss3dRenderer, Renderer},
    scene::{
        game_object::{GameObject, INFINITY},
        PhysicsScene,
    },
};

fn main() {
    //Initialize renderer & scene

    let mut renderer = Kiss3dRenderer::new("Demo", 1200, 900);
    let mut scene = PhysicsScene::new();

    //Create spheres in a cube formation using some default values
    //The amount of spheres in a row in the formation
    let width = 4;
    //The radii of the individual spheres
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

                //Add some different velocities to the spheres
                sphere.add_velocity([x as f32 - 11.5, y as f32 - 12.3, z as f32 - 11.2]);

                //remember to add gameobject to scene and renderer at the same time to make sure they are synced.
                renderer.add_obj(&sphere).unwrap();
                scene.add(sphere);
            }
        }
    }

    //Create planes using some default values, for walls and floor
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

    //Change the camera speed and position to work well with the scale of the Scene
    renderer.change_camera_speed(0.1);
    renderer.camera_position([0., 10., 0.]);
    renderer.set_background(0.5, 0.5, 0.5);

    //Main loop
    loop {
        //Increment time and simulate physics
        scene.update(0.007);
        //Draw changes
        renderer.draw(&scene.objects()).unwrap();
    }
}
