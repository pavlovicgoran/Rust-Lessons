// We create a tests directory at the top level of our project directory, next to src. 
// Cargo knows to look for integration test files in this directory. 
// We can then make as many test files as we want to in this directory, 
// and Cargo will compile each of the files as an individual crate.
use adder;

#[test]
fn test_rectangle_fits() {
    let larger = adder::Rectangle {
        width: 6,
        height: 6,
    };
    let smaller = adder::Rectangle {
        width: 3,
        height: 3,
    };
    assert_eq!(larger.can_hold(&smaller), true);
}