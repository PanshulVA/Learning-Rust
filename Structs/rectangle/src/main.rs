// fn main() {
//     let width=28;
//     let height=57;
//     println!(
//         "the area of triangle is {} sqaure pixels",
//         area(width, height)
//     );
// }
// fn area(a: u32 , b: u32) -> u32 {
//     a*b
// }

// //USING TUPLES
// fn main() {
//     let rect=(28,57);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect)
//     );
// }

// fn area(dimensions: (u32,u32)) -> u32 {
//     dimensions.0*dimensions.1
// }

// //USING STRUCT
// struct Rectangle {
//     width:u32,
//     height:u32,
// }
// fn main() {
//     let rect1 = Rectangle {
//         width:28,
//         height:57,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width*rect.height
// }

//

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    // println!("rect1 is {rect1:?}");
}