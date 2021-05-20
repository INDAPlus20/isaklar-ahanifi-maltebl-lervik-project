use kiss3d::{
    camera::FirstPerson,
    event::Key,
    nalgebra::{Point3, Translation3, UnitQuaternion, Vector3},
    scene::SceneNode,
    window::Window,
};

use crate::{scene::game_object::GameObject, shapes::shape::Shape};

mod tests;

pub const PLANE_SIZE: f32 = 10000.;

pub trait Renderer {
    fn draw(&mut self, gameobjects: &[GameObject]) -> Result<(), String>;

    fn add_obj(&mut self, object: &GameObject) -> Result<(), String>;

    fn remove_obj(&mut self, index: usize) -> Result<(), String>;
}

/// Renderer struct for using Kiss3D.
/// Adding and removing objects must be done in sync with Scene.
pub struct Kiss3dRenderer {
    window: Window,
    camera: FirstPerson,
    // Objects must be in same order as in Scene
    renderables: Vec<SceneNode>,
}

impl Kiss3dRenderer {
    /// Add basic rendering window with size and title and free moving
    /// Camera using move_step 0.1 & wasd-keys as well as
    /// rotation and movement holding left/right mouse button
    pub fn new(title: &str, width: u32, height: u32) -> Kiss3dRenderer {
        let mut camera = kiss3d::camera::FirstPerson::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.3, 0.2, 1.0),
        );

        camera.set_move_step(0.1);
        camera.rebind_left_key(Some(Key::A));
        camera.rebind_right_key(Some(Key::D));
        camera.rebind_up_key(Some(Key::W));
        camera.rebind_down_key(Some(Key::S));

        let mut window = kiss3d::window::Window::new_with_size(title, width, height);
        window.set_light(kiss3d::light::Light::StickToCamera);
        Kiss3dRenderer {
            window,
            camera,
            renderables: Vec::new(),
        }
    }
    ///Change keybindings and speed of camera
    pub fn set_camera_movement(
        &mut self,
        move_step: f32,
        up: Key,
        down: Key,
        left: Key,
        right: Key,
    ) {
        self.camera.unbind_movement_keys();
        self.camera.set_move_step(move_step);
        self.camera.rebind_left_key(Some(left));
        self.camera.rebind_right_key(Some(right));
        self.camera.rebind_up_key(Some(up));
        self.camera.rebind_down_key(Some(down));
    }
    ///Change the distance the camera moves with a single key press
    pub fn change_camera_speed(&mut self, move_step: f32) {
        self.camera.set_move_step(move_step);
    }

    ///Change the background colour
    pub fn set_background(&mut self, r: f32, g: f32, b: f32) {
        self.window.set_background_color(r, g, b);
    }

    ///Set the one global light source allowed by kiss3d to a point
    pub fn set_point_light_source(&mut self, point: Point3<f32>) {
        self.window.set_light(kiss3d::light::Light::Absolute(point));
    }
    ///Set the one global light source allowed by kiss3d to follow the camera.
    pub fn set_light_to_camera(&mut self) {
        self.window.set_light(kiss3d::light::Light::StickToCamera);
    }

    ///Match a Shape with a Scene Node to render
    fn node_from_shape(&mut self, shape: &dyn Shape) -> SceneNode {
        if let Ok(sphere) = shape.as_sphere() {
            self.window.add_sphere(sphere.radius)
        } else if let Ok(cube) = shape.as_cube() {
            let extents = cube.half_extents * 2.0;
            self.window.add_cube(extents.x, extents.y, extents.z)
        } else if let Ok(plane) = shape.as_plane() {
            let mut g = self.window.add_group();
            let mut p = g.add_quad(PLANE_SIZE, PLANE_SIZE, 1, 1);
            p.append_rotation(
                &UnitQuaternion::rotation_between(&Vector3::z(), plane.normal()).unwrap(),
            );
            p.append_translation(&Translation3::new(0., -1., 0.));
            g
        } else {
            panic!()
        }
        //TODO: Add more shapes
    }
}

impl Renderer for Kiss3dRenderer {
    /// Draw function to call at each frame update.
    fn draw(&mut self, gameobjects: &[GameObject]) -> Result<(), String> {
        let mut i = 0;
        if self.window.render_with_camera(&mut self.camera) {
            //Sync position of objects by setting the position of rendered object
            //to the position of GameObject
            for go in gameobjects {
                self.renderables[i].set_local_transformation(go.position);
                i += 1;
            }
            return Ok(());
        }
        Err(String::from("Error: Window closed for rendering"))
    }

    /// Add a rendered SceneNode to the renderer
    fn add_obj(&mut self, object: &GameObject) -> Result<(), String> {
        let mut new_node = self.node_from_shape(object.shape());
        let [r, g, b] = object.color();
        new_node.set_color(r as f32 / 255., g as f32 / 255., b as f32 / 255.);
        self.renderables.push(new_node);
        Ok(())
    }

    /// Remove a rendered SceneNode by index from the renderer
    fn remove_obj(&mut self, index: usize) -> Result<(), String> {
        self.renderables.remove(index);
        Ok(())
    }
}
