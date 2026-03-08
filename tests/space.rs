use rray::space::Vector;

///
/// test vector dot product calculation
///
#[test]
fn vector_dot_product() {
    let v0 = Vector::new(1.0, 2.0, 3.0);
    let v1 = Vector::new(2.0, 3.0, 4.0);

    assert_eq!(v0.dot(&v1), 20.0);

    let v0 = Vector::new(1.0, 0.0, 0.0);
    let v1 = Vector::new(0.0, 0.0, 4.0);

    assert_eq!(v0.dot(&v1), 0.0);
}

///
/// test vector cross product calculation
///
#[test]
fn vector_cross_product() {
    let v0 = Vector::new(1.0, 0.0, 0.0);
    let v1 = Vector::new(0.0, 0.0, 1.0);

    assert_eq!(v0.cross(&v1), Vector::new(0.0, -1.0, 0.0));
    assert_eq!(v1.cross(&v0), Vector::new(0.0, 1.0, 0.0));

    let v0 = Vector::new(2.0, 6.0, 3.0);
    let v1 = Vector::new(1.0, 4.0, 5.0);

    assert_eq!(v0.cross(&v1), Vector::new(18.0, -7.0, 2.0));
    assert_eq!(v1.cross(&v0), Vector::new(-18.0, 7.0, -2.0));
}
