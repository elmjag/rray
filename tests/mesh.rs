use rray::{mesh::Vertex, vector::Vector};

///
/// test Vertex substraction
///
#[test]
fn vertex_sub() {
    let v0 = Vertex::new(1.0, 2.0, 3.0);
    let v1 = Vertex::new(4.0, 6.0, 2.0);

    assert_eq!(&v0 - &v1, Vector::new(-3.0, -4.0, 1.0));
    assert_eq!(&v1 - &v0, Vector::new(3.0, 4.0, -1.0));
}
