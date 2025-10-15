use adder::add;

mod common;

// to run only integration tests: cargo test --test integration_test
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(add(2, 2), 4);
}
