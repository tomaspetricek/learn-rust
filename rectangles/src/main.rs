#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // asscociated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // is like a static method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width, height)
    );
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 10,
    };
    let _area = area2(&rect1);

    println!("rect1 is {rect1:#?}");

    dbg!(&rect1);

    let _area = rect1.area();
    println!("The area of the rectangle: {rect1:?} is {_area}");

    let square = Rectangle::square(4);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
