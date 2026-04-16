use std::f32::consts::TAU;

use rray::{
    math::is_close,
    rotation::Rotation,
    space::{Vector, Vertex},
};

/// Assert that vertices v1 and v2
fn assert_is_close(v1: Vertex, v2: Vertex) {
    assert!(is_close(v1.x(), v2.x()));
    assert!(is_close(v1.y(), v2.y()));
    assert!(is_close(v1.z(), v2.z()));
}

#[test]
fn zero_angle_rotation() {
    let rotation = Rotation::new(0.0, &Vector::new(1.0, 0.0, 0.0));
    let vtx = Vertex::new(0.0, 0.0, 0.0);

    // the rotated vertex should be same as input vertex
    let result = rotation.apply(&vtx);
    assert_is_close(vtx, result);
}

#[test]
fn around_y_axis() {
    //
    // rotations around Y axis
    //
    let vtx = Vertex::new(1.0, 2.0, 0.0);
    let y_axis = Vector::new(0.0, 1.0, 0.0);

    // 45° rotation
    let angle = TAU / 8.0;
    let rot = Rotation::new(angle, &y_axis);

    assert_is_close(rot.apply(&vtx), Vertex::new(0.707106781, 2.0, 0.707106781));

    // -45° rotation
    let angle = -(TAU / 8.0);
    let rot = Rotation::new(angle, &y_axis);

    assert_is_close(rot.apply(&vtx), Vertex::new(0.707106781, 2.0, -0.707106781));

    // 90° rotation
    let angle = TAU / 4.0;
    let rot = Rotation::new(angle, &y_axis);

    assert_is_close(rot.apply(&vtx), Vertex::new(0.0, 2.0, 1.0));
}

#[test]
#[should_panic(expected = "non-unit rotation axis vector")]
fn non_unit_rotation_axis() {
    Rotation::new(0.0, &Vector::new(1.1, 0.0, 0.0));
}
