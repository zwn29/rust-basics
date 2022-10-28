#![deny(clippy::all)]

fn main() {
    let personal_data = (22, "john");
    // let (age, name) = personal_data;
    let age = personal_data.0;
    let name = personal_data.1;
    println!("{} {}", name, age);
}
