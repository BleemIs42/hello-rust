#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = rect1;
    println!("react1 {:#?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let sq = Rectangle::square(3);
    println!(
        "The area of the rectangle is {:#?} square pixels.",
        sq
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}