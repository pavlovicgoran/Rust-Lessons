mod front_of_house;
// Bringing path into the scope
pub use crate::front_of_house::hosting;
fn serve_order() {}

mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        // Calling a function using a relative path
        super::serve_order();
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,       // This variable is public
        seasonal_fruits: String, // This variable is private
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toase please", meal.toast);

    // next line won't compile if not commented => the seasonal fruits variable is private
    //meal.seasonal_fruits = String::from("blueberries");

    let soup = back_of_house::Appetizer::Soup;
    let salad = back_of_house::Appetizer::Salad;
}
