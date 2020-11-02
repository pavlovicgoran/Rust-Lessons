// The purpose of unit tests is to test each unit of code in isolation from the rest of the code
// to quickly pinpoint where code is and isn’t working as expected. 
// You’ll put unit tests in the src directory in each file with the code that they’re testing. 
// The convention is to create a module named tests in each file to contain the test functions 
// and to annotate the module with cfg(test).
pub struct Guess {
    value: i32,
}
    
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
    }
    
        Guess { value }
    }
}

#[cfg(test)] // tells compiler to run the test code only on cargo test
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        // test for panic!
        Guess::new(200);
    }
}