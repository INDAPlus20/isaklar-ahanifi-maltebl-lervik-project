use super::*;
use crate::shapes::sphere::Sphere;
use kiss3d::nalgebra::{Point, Point3, Rotation3, Translation3, UnitQuaternion};

#[test]
fn sphere_sphere_collision_check() {
    let sphere_1 = Sphere::new(2.0f32);
    let sphere_2 = Sphere::new(2.0f32);
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(0f32, 2f32, 2f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let result = sphere_sphere(&sphere_1, &sphere_2, &iso_1, &iso_2);
    assert_eq!(result, true)
}

#[test]
fn sphere_manifold() {
    let sphere_1 = Sphere::new(2.0f32);
    let sphere_2 = Sphere::new(2.0f32);
    let iso_1 = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let iso_2 = Isometry3::from_parts(
        Translation3::new(0f32, 3f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let result = CollisionManifold::sphere_sphere(&sphere_1, &sphere_2, &iso_1, &iso_2);
    let mut contacts = Vec::with_capacity(10);
    contacts.push(Point3::new(0f32, 1.5f32, 0f32));
    let facit = CollisionManifold {
        colliding: true,
        depth: 0.5f32,
        normal: UnitVector3::new_normalize(Vector3::new(0f32, 1f32, 0f32)),
        contacts: contacts,
    };
    assert_eq!(facit, result);
}

#[test]
fn sphere_plane_collision_check() {
    let mut sphere = Sphere::new(2.1f32);
    let plane = Plane::new(UnitVector3::new_normalize(Vector3::new(0.0, 1.0, 0.0)));
    let iso_s = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );
    let mut iso_p = Isometry3::from_parts(
        Translation3::new(0f32, 2f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::TAU),
    );
    let result = sphere_plane(&sphere, &plane, &iso_s, &iso_p);
    assert_eq!(result, true);

    sphere.radius = 1.9;
    let result = sphere_plane(&sphere, &plane, &iso_s, &iso_p);
    assert_eq!(result, false);

    iso_p = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_2_PI),
    );

    let result = sphere_plane(&sphere, &plane, &iso_s, &iso_p);
    assert_eq!(result, true);

    iso_p = Isometry3::from_parts(
        Translation3::new(0f32, 2f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::PI),
    );
    let result = sphere_plane(&sphere, &plane, &iso_s, &iso_p);
    assert_eq!(result, false);
}

#[test]
fn sphere_plane_manifold() {
    let mut sphere = Sphere::new(2.1f32);
    let iso_s = Isometry3::from_parts(
        Translation3::new(0f32, 1f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::FRAC_PI_2),
    );

    let plane = Plane::new(UnitVector3::new_normalize(Vector3::new(0.0, 1.0, 0.0)));
    let mut iso_p = Isometry3::from_parts(
        Translation3::new(0f32, 0f32, 0f32),
        UnitQuaternion::new(Vector3::y() * std::f32::consts::TAU),
    );

    let result = CollisionManifold::sphere_plane(&sphere, &plane, &iso_s, &iso_p);

    let test = CollisionManifold {
        colliding: true,
        normal: UnitVector3::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
        depth: 0.099999905, // Possible rounding errors somewhere
        contacts: vec![Point3::new(0.0, 2.0, 0.0)],
    };
    assert_eq!(test, result);
}
