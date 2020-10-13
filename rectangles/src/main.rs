#[derive(Debug)]  // this trait makes debugging easy with "{:?}" and "{:#?}"
struct Rectangle {
    width: u32,
    height: u32,
}
// methods take &self or &mut self to work with the instance of the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height >= rectangle.height && self.width >= rectangle.width
    }
}
// Associated functions don't take self as parameter
// They are usually convenient initializers as below
impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 200,
        height: 400
    };
    let smaller_rect = Rectangle {
        width: 100,
        height: 100,
    };
    let bigger_rect: Rectangle = Rectangle {
        width: 300,
        height: 300,
    };
    println!("Area of Rectangle is: {}", rect.area());
    println!("Rectangle {:?} can hold rectangle {:?} = {}", rect, smaller_rect, rect.can_hold(&smaller_rect));
    println!("Rectangle {:?} can hold rectangle {:?} = {}", rect, bigger_rect, rect.can_hold(&bigger_rect));

    let square = Rectangle::square(150);
    println!("Square: {:#?}", square);
}
