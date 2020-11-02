// Module needs to be included in main.rs or lib.rs
// in order to be compiled
mod guessing;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // assert! checks only if the parameter it got is true
        // it doesn't print the values of the parameters
        // you should use assert_eq!(...) and asser_ne!(...) since they
        // can help you to see why the test has failed
        // assert!(larger.can_hold(&smaller));
        assert_eq!(larger.can_hold(&smaller), true);
    }

    #[test]
    // #[ignore] - add to test you want to exclude, test that is expensive -> takes a lot to execute
    // you can run the ignored tests with "cargo test -- --ignored"
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // asserts can have custom messages to help with the debugging
        assert_ne!(
            smaller.can_hold(&larger),
            true,
            "Smaller Rectangle cannot contain larger Rectangle",
        );
    }
}
