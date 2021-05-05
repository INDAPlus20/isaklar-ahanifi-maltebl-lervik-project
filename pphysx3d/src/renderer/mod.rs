use std::usize;

use kiss3d::{scene::SceneNode, window::Window};

use crate::shapes::GameObject;

mod tests;

pub trait Renderer {
    fn draw(&mut self, gameobjects: &[GameObject]) -> Result<(), String>;

    fn add_obj(&mut self, object: &GameObject) -> Result<(), String>;

    fn remove_obj(&mut self, index: usize) -> Result<(), String>;
}

pub struct Kiss3dRenderer {
    window: Window,
    renderables: Vec<SceneNode>,
}

impl Kiss3dRenderer {
    /// Add basic rendering window with size and title.
    pub fn new(title: &str, width: usize, height: usize) -> Kiss3dRenderer {
        let mut window = kiss3d::window::Window::new_with_size(title, 600, 300);
        window.set_light(kiss3d::light::Light::StickToCamera);
        Kiss3dRenderer {
            window,
            renderables: Vec::new(),
        }
    }
}

impl Renderer for Kiss3dRenderer {
    fn draw(&mut self, gameobjects: &[GameObject]) -> Result<(), String> {
        let mut i = 0;
        if self.window.render() {
            for go in gameobjects {
                self.renderables[i].set_local_transformation(go.position);
                i += 1;
            }
            return Ok(());
        }
        Err(String::from("Error: Window closed for rendering"))
    }

    /// Add a rendered SceneNide to the renderer
    fn add_obj(&mut self, object: &GameObject) -> Result<(), String> {
        let sphere = object
            .shape
            .as_sphere()
            .ok_or_else(|| String::from("Could not add object"))?;
        let mut new_sphere = self.window.add_sphere(sphere.radius);
        new_sphere.set_color(1.0, 0.0, 0.0);
        self.renderables.push(new_sphere);
        Ok(())
    }

    /// Remove a rendered SceneNode by index from the renderer
    fn remove_obj(&mut self, index: usize) -> Result<(), String> {
        self.renderables.remove(index);
        Ok(())
    }
}
