struct Rectangle {
    length: i32,
    width: i32,
}

fn assign_rectangle(length: i32, width: i32) -> Rectangle {
    Rectangle {
        length,
        width,
    }
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.length
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.length > rect.length
    }
}

fn main () {
    let rectangle1 = assign_rectangle(13, 20);
    let rectangle2: Rectangle = assign_rectangle(9, 10);
    
    println!("The area of rectanlge 1 is: {}", rectangle1.area());
    println!("The area of rectangle 2 is: {}", rectangle2.area());

    println!("{}", rectangle1.can_hold(&rectangle2));
}