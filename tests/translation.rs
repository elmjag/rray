use rray::space::Vertex;
use rray::translation::Translation;

mod utils;
use utils::vertices_are_close;

//
// test applying translation to a vertex
//
#[test]
fn apply() {
    let t = Translation::new(1.0, -1.0, 0.5);
    assert!(vertices_are_close(
        t.apply(Vertex::new(3.0, 2.0, 1.0)),
        Vertex::new(4.0, 1.0, 1.5)
    ));

    // test zero translation
    let t = Translation::new(0.0, 0.0, 0.0);
    assert!(vertices_are_close(
        t.apply(Vertex::new(12.0, -12.0, 4.0)),
        Vertex::new(12.0, -12.0, 4.0)
    ));
}
