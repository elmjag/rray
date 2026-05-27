use std::f32::consts::TAU;

mod utils;
use utils::vertices_are_close;

use rray::{
    rotation::Rotation,
    space::{Vector, Vertex},
};

#[test]
fn zero_angle_rotation() {
    let rotation = Rotation::new(0.0, &Vector::new(1.0, 0.0, 0.0));
    let vtx = Vertex::new(0.0, 0.0, 0.0);

    // the rotated vertex should be same as input vertex
    let result = rotation.apply(&vtx);
    assert!(vertices_are_close(vtx, result));
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

    assert!(vertices_are_close(
        rot.apply(&vtx),
        Vertex::new(0.707106781, 2.0, 0.707106781)
    ));

    // -45° rotation
    let angle = -(TAU / 8.0);
    let rot = Rotation::new(angle, &y_axis);

    assert!(vertices_are_close(
        rot.apply(&vtx),
        Vertex::new(0.707106781, 2.0, -0.707106781)
    ));

    // 90° rotation
    let angle = TAU / 4.0;
    let rot = Rotation::new(angle, &y_axis);

    assert!(vertices_are_close(
        rot.apply(&vtx),
        Vertex::new(0.0, 2.0, 1.0)
    ));
}

#[test]
#[should_panic(expected = "non-unit rotation axis vector")]
fn non_unit_rotation_axis() {
    Rotation::new(0.0, &Vector::new(1.1, 0.0, 0.0));
}
