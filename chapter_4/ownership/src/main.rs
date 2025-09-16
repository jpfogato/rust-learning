fn read(y:bool) {
        if y {
                println!("y is true!");
        }  
}

fn main () {
        let x = true;
        read(x);

        {
                // In the Stack:
                // The value of a is copied into b, and a is left unchanged, even after changing b. 
                let a = 5;
                let mut b = a;
                b += 1;
        }

        {
                /*
                At L1, the string “Ferris” has been allocated on the heap. It is owned by first.
                At L2, the function add_suffix(first) has been called. This moves ownership of the string from first to name. The string data is not copied, but the pointer to the data is copied.
                At L3, the function name.push_str(" Jr.") resizes the string’s heap allocation. This does three things. First, it creates a new larger allocation. Second, it writes “Ferris Jr.” into the new allocation. Third, it frees the original heap memory. first now points to deallocated memory.
                At L4, the frame for add_suffix is gone. This function returned name, transferring ownership of the string to full.
                */

                let first = String::from("Ferris"); // L1
                let full = add_suffix(first); // L4
                println!("{full}");
                // after this, we can no longer use "first" on our code because it has been removed from the stack and "full" now owns the pointer to the heap

        }


}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr."); // L2 and L3
    name
}