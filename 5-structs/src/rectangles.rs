#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Define methods for Rectangle
impl Rectangle {
    // Note that we still need to use the & in front of the self shorthand to indicate this method borrows the Self instance,
    // just as we did in "rectangle: &Rectangle" in area()
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height  > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    // The ":?" specifier tells println! to use the Debug output format
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub fn main2() {
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);

    println!("The area of the rectangle is {} square pixels", rect2.area());
}

pub fn main3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
