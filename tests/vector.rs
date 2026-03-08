use rray::vector::Vector;

///
/// test vector dot product calculation
///
#[test]
fn dot_product() {
    let v0 = Vector::new(1.0, 2.0, 3.0);
    let v1 = Vector::new(2.0, 3.0, 4.0);

    assert_eq!(v0.dot(&v1), 20.0);

    let v0 = Vector::new(1.0, 0.0, 0.0);
    let v1 = Vector::new(0.0, 0.0, 4.0);

    assert_eq!(v0.dot(&v1), 0.0);
}
