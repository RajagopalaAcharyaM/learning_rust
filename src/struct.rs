#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

struct Colour(i64, u64,u64);
struct Point(i32, i32, i32);

fn build_user (username: String, email: String, active: bool, sign_in_count: i64) -> User {
    User {
        active,
        username,
        email,
        sign_in_count,
    }
}
fn main () {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("acharyarajagopala@gmail.com"),
    //     email: String::from("acharyarajagopala@gmail.com"),
    //     sign_in_count: 1,

    // };
    let name = String::from("Rajagopala Acharya");
    let email = String::from("acharyarajagopala@gmail.com");
    let user1 = build_user(name, email, true, 2);
    // let user2 = build_user(user1.username.clone(), String::from("hellogmail"), user1.active.clone(), user1.sign_in_count.clone());
    // println!("{}, {}", user1.username, user2.email);
    let user2 = User {
        email: String::from("ninnajjipinda@gmail.com"),
        ..user1.clone()
    };
    println!("{},{}", user1.username, user2.username);

    let red = Colour(250, 0, 0);
    let zero: Point = Point(0, 0, 0);

    println!("The colour {}, {}, {} is at the point {}, {}, {}", red.0, red.1, red.2, zero.0, zero.1, zero.2);
}