#[cfg(test)]
use super::bounding_volume::BoundingVolume;
#[cfg(test)]
use crate::shapes::sphere::Sphere;

#[cfg(test)]
use crate::shapes::cube::Cube;

#[cfg(test)]
use kiss3d::nalgebra::{Isometry3, Vector3};
use kiss3d::ncollide3d::math::Vector;

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
#[allow(non_snake_case)]
fn test_AABB_around_sphere() {
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
#[allow(non_snake_case)]
fn test_AABB_around_non_rotated_cube() {
    let half_extents = Vector3::new(1.0, 1.0, 1.0);
    let cube_medium = Cube::new(half_extents);
    let half_extents = Vector3::new(0.5, 0.5, 0.5);
    let cube_small = Cube::new(half_extents);

    let axisangle = Vector3::y() * 0.0; //the direction of the vector is the axis of rotation and the magnitude is the amount of rotation in radians
    let translation = Vector3::new(0.0, 0.0, 0.0);

    let zero_pos = Isometry3::new(translation, axisangle);

    let translation = Vector3::new(10.0, 0.0, 0.0);
    let non_colliding_pos = Isometry3::new(translation, axisangle);

    //println!("{}",zero_pos.rotation.to_rotation_matrix());

    let is_intersecting = cube_medium
        .aabb(&zero_pos)
        .interects(&cube_small.aabb(&non_colliding_pos));
    assert_eq!(false, is_intersecting);

    let translation = Vector3::new(-1.0, 0.0, 0.0);
    let pos_to_left = Isometry3::new(translation, axisangle);

    let is_intersecting = cube_medium
        .aabb(&zero_pos)
        .interects(&cube_small.aabb(&pos_to_left));
    assert_eq!(true, is_intersecting);
}

#[test]
#[allow(non_snake_case)]
fn test_AABB_around_rotated_cube() {
    let half_extents = Vector3::new(1.0, 1.0, 1.0);
    let cube = Cube::new(half_extents);

    let axis_angle = Vector3::x() * std::f32::consts::FRAC_PI_4; //rotate 45 degrees around the x axis
    let translation = Vector3::new(0.0, 0.0, 0.0);

    let rotated_pos = Isometry3::new(translation, axis_angle);

    let axis_angle = Vector3::y() * 0.0;
    let translation = Vector3::new(-1.75, 0.0, 1.75);

    let mut pos = Isometry3::new(translation, axis_angle);

    let is_intersecting = cube.aabb(&rotated_pos).interects(&cube.aabb(&pos));
    assert_eq!(true, is_intersecting);

    pos.append_translation_mut(&Vector3::new(-0.3, 0.0, 0.0).into());

    let is_intersecting = cube.aabb(&rotated_pos).interects(&cube.aabb(&pos));
    assert_eq!(false, is_intersecting);
}
