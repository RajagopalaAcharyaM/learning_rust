fn main() {
    let mut string1 = String::from("Hello");
    let string2 = String::from(", World");
    string1 = push_string(&mut string1, &string2);
    println!("{string1}");
    let string3 = String::from(&string1);
    let string2 = string1;
    // let string4 = string1;
    println!("{string2}");
    println!("{string3}");
}

fn push_string( some_string: &mut String, string_to_be_pushed: &String) -> String{
    // let mut return_string = some_string;
    // return_string.push_str(string_to_be_pushed);
    // return_string
    some_string.push_str(string_to_be_pushed);
    let return_string = String::from(some_string.clone());
    return_string
}