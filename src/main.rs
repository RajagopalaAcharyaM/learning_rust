fn first_word(string: &String) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &string[0..i];
        }
    }

    &string[..]
}

fn main() {
    let s = String::from("My name is Raj");
    let first_word = first_word(&s);
    println!("{first_word}");
}