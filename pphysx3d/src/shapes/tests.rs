use kiss3d::nalgebra::{self as na, Point, Point3, Translation3, Vector3};

use crate::shapes::sphere::Sphere;

use super::bounding_volume::BoundingVolume;


#[test]
fn test_bounding_sphere_around_sphere(){
    let sphere_big = Sphere::new(2f32,Point3::new(0f32,0f32,0f32));
    let sphere_medium = Sphere::new(1f32, Point3::new(0f32,0f32,0f32));
    let sphere_small = Sphere::new(0.5f32, Point3::new(0f32,0f32,0f32));

    assert!(sphere_medium.bounding_sphere().contains(&sphere_small.bounding_sphere())==true);
    assert!(sphere_medium.bounding_sphere().contains(&sphere_big.bounding_sphere())==false);
}

#[test]
fn test_AABB_sphere_around_sphere(){
    let sphere_big = Sphere::new(2f32,Point3::new(0f32,0f32,0f32));
    let sphere_medium = Sphere::new(1f32, Point3::new(0f32,0f32,0f32));
    let sphere_small = Sphere::new(0.5f32, Point3::new(0f32,0f32,0f32));

    assert!(sphere_medium.aabb().contains(&sphere_small.aabb())==true);
    assert!(sphere_medium.aabb().contains(&sphere_big.aabb())==false);
}


#[test]
fn test_cube_render() {
    let mut window = kiss3d::window::Window::new_with_size("kiss3d: cube", 600, 300);
    let mut cube = window.add_cube(0.4, 0.4, 0.4);

    cube.set_color(1.0, 0.0, 0.0);

    window.set_light(kiss3d::light::Light::StickToCamera);

    let rotation = na::UnitQuaternion::from_axis_angle(&na::Vector3::y_axis(), 0.014);
    while window.render() {
        cube.append_rotation(&rotation);
        cube.append_translation(&Translation3::new(0.0, 0.0, 0.1));
    }
}


