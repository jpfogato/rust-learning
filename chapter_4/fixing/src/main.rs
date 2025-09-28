// Nothing here works... and it is fine :)

fn main() {}

fn return_a_string() -> &String {
    /*
    This function is unsafe because “Data Must Outlive All Of Its References”

    To fix this function:
    remove borrows from function (&)
    Return a reference to a static string: &'static str
    Other options: Use garbage collector (std::rc::Rc) or
    Let the caller create a slot for the string by using "output: &mut String"
    */
    let s = String::from("Hello world");
    &s
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    /*
    This function is rejected by the borrow checker because "name" is immutable

    To fix this function:
    (Not good!) parse a mutable reference as parameter using &mut Vec<String>
    (Not good!) take ownership of the name, by changing &Vec<String> to Vec<String>
    (Better) clone the "name" input into a new variable.
    */
    name.push(String::from("Esq.")); // Requires W permission
    let full = name.join(" ");
    full
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    /*
    This function is rejected by the borrow checker because "dst.push(..)" could deallocate the contents of "dst", invalidating the reference "largest"

    To fix this function:
    (Not good!) clone dst. Allocating new memory causes performance hit
    (Not good!) Perform all the length comparisons first, and then mutate dst afterwards. Allocating new memory causes performance hit
    Copy out the length of largest, since we don’t actually need the contents of largest, just its length.
    */
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // let largest = .. removes the W permissions on dst
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone()); // However, dst.push(..) requires the W permission
        }
    }
}
