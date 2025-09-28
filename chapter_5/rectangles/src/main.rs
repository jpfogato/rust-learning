#[derive(Debug)] // this outer attribute that allows println! of Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("this output uses {{rect1:?}} -> {rect1:?}");
    println!("this output uses {{rect1:#?}} -> {rect1:#?}");

    dbg!(&rect1); // we do not want dbg! to gain ownership of rect1, so we use &
    // dbg! uses 'stderr' instead of 'stdout' like println!
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
