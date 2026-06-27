#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width*self.height
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 28,
//         height: 57,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 28,
        height: 57,
    };
    let rect2 = Rectangle {
        width: 16,
        height: 44,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 22,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}