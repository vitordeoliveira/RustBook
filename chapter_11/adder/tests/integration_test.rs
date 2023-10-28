use adder;

mod commom;

#[test]
fn it_adds_two() {
    commom::setup();
    assert_eq!(4, adder::add_two(2));
}
