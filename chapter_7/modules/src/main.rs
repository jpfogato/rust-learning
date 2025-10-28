use crate::garden::vegetables::Asparagus; // Asparagus is now callable!

pub mod garden; // tells the compiller to include whatever code it finds in the garden crate

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
