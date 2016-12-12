extern crate webserver_rust as webserver;

#[test]
fn integration_test_works() {
    assert_eq!(4, webserver::add_two(2));
}