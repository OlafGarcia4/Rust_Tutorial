#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        self.width * 2 + self.height * 2
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let a = Rectangle::new(10, 20);
    let b = Rectangle::new(10, 10);

    println!("\n\nRectangle A: {:?}", a);
    println!("Rectangle B: {:?}", b);
    let mut temp = a.area();
    println!("A's area: {:?}", temp);
    temp = a.perimeter();
    println!("A's perimeter: {:?}", temp);
    let canIt = a.can_hold(&b);
    println!("A can hold B: {:?}", canIt);
}
