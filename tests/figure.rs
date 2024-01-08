use tetris_rust::figure::Figure;

#[test]
fn works() {
    let f = Figure::new(0, 0, 0, 0);

    assert!(f.is_here(0, 0));
    assert!(f.is_here(1, 0));
    assert!(f.is_here(2, 0));
    assert!(f.is_here(3, 0));
}
