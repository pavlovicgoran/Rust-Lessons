mod traits;
mod lifetimes;

// Example of Generic Function
// Type is restricted to implement std::cmp::PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, &100);

    let char_list = vec!['1', 'b', 'd', 'q'];


    let result = largest(&char_list);
    println!("The largest ASCII char is {}", result);

    let result = lifetimes::longest("abcd", "cds");
    println!("Result: {}", result);
}
