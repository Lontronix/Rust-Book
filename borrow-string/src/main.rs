// MARK: ownership

// fn main() {
//     let mut name = String::from("Lonnie");
//
//     mutate_name(name);
//     println!("{name}");
// }
//
// fn mutate_name(mut name: String) -> String {
//     name.push_str(" gerol");
//     name
// }

// MARK: Borrowing
fn main() {
    let mut name = String::from("Lonnie");

    mutate_name(&mut name);
    println!("{name}");
}

fn mutate_name(name: &mut String) {
    name.push_str(" gerol");
}
