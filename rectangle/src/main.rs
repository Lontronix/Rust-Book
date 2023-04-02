// MARK: Inital
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(width1, height1)
//     );
//
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// MARK: with Tuples
// fn main() {
//     let rect = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         compute_area(rect)
//     )
// }
//
// fn compute_area(rect: (u32, u32)) -> u32 {
//     return rect.0 * rect.1
// }


//MARK: with struct

// conforms to the debug trait with a default implementation meant for debugging
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    // {:? means use the debug trait to print out this value (the Debug trait has a fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result method that gets called)
    println!("rect: {:?}", rect);

    println!(
        "The area is {} square pixels",
        compute_area(rect)
    );

}

fn compute_area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}
