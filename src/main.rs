use chrono::NaiveDate;

struct User {
    name: String,
    email: String,
    project: String,
    date_of_joining: NaiveDate,
}

fn add_employee(name: &String, email: &String, project: &String, date: &NaiveDate) -> User {
    User {
        name: name.clone(),
        email: email.clone(),
        project: project.clone(),
        date_of_joining: date.clone(),
    }
}

fn main(){
    let mut name = String::from("Rajagopala Acharya");
    let mut email: String = String::from("acharyarajagopala@gmail.com");
    let mut project: String = String::from("Personal bojja");
    let mut date_string: String = String::from("2024-12-08");
    let mut naive_date: NaiveDate = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").unwrap();
    let mut user = add_employee(&name, &email, &project, &naive_date);
    println!("Can I access date string?: {}", date_string);
    println!("Can I access name?: {}", name);
    println!("Date of the employee joining: {}", user.date_of_joining);
}