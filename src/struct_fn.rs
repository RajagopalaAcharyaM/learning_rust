// struct Rectangle {
//     length: i32,
//     width: i32,
// }

// fn assign_rectangle(length: i32, width: i32) -> Rectangle {
//     Rectangle {
//         length,
//         width,
//     }
// }

// fn find_area (rectangle: &Rectangle) -> i32 {
//     let area = rectangle.length * rectangle.width;
//     return area;
// }

// fn main () {
//     let length: i32 = 34;
//     let width: i32 = 45;
//     let rectangle1 = assign_rectangle(length, width);
//     let area_of_rectangle = find_area(&rectangle1);
//     println!("The length and the width of the recatngle are {}, {}", rectangle1.length, rectangle1.width);
//     println!("The area of the rectangle are {}", area_of_rectangle);
//     println!("{}", length);
// }