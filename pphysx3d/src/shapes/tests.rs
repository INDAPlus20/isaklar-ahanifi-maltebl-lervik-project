use kiss3d::nalgebra::{self as na, Point, Point3, Translation3, Vector3};
use na::Isometry3;

use crate::shapes::sphere::Sphere;

use super::bounding_volume::BoundingVolume;

#[test]
fn test_bounding_sphere_around_sphere() {
    let sphere_big = Sphere::new(2f32);
    let sphere_medium = Sphere::new(1f32);
    let sphere_small = Sphere::new(0.5f32);

    let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_2;
    let translation = Vector3::new(1.0, 2.0, 3.0);
    let isometry = Isometry3::new(translation, axisangle);

    assert!(
        sphere_medium
            .bounding_sphere(&isometry)
            .contains(&sphere_small.bounding_sphere(&isometry))
            == true
    );
    assert!(
        sphere_medium
            .bounding_sphere(&isometry)
            .contains(&sphere_big.bounding_sphere(&isometry))
            == false
    );
}

#[test]
fn test_AABB_sphere_around_sphere() {
    let sphere_big = Sphere::new(2f32);
    let sphere_medium = Sphere::new(1f32);
    let sphere_small = Sphere::new(0.5f32);

    let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_2;
    let translation = Vector3::new(1.0, 2.0, 3.0);
    let isometry = Isometry3::new(translation, axisangle);

    assert!(
        sphere_medium
            .aabb(&isometry)
            .contains(&sphere_small.aabb(&isometry))
            == true
    );
    assert!(
        sphere_medium
            .aabb(&isometry)
            .contains(&sphere_big.aabb(&isometry))
            == false
    );
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
