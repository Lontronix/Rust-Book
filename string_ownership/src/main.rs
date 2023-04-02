// MARK: String ownership
// Using this will result in error[E0382]: borrow of moved value: `name` after calling print_string in main

// fn print_string(str: String) {
//     println!("accessing string from print_string: {}", str);
// }


// MARK: String reference

fn print_string(str: &str) {
    println!("accessing string from print_string: {str}");
}


fn main() {
    let name = String::from("Lontronix");
    print_string(&name);
    println!("accessing string from main: {name}");
}
