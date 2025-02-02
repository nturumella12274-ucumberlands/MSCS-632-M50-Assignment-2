fn main() {
    // Stack allocation (primitive type)
    let x = 5;
    let y = x;  // Copy semantics for i32 (on the stack)
    println!("x = {}, y = {}", x, y);

    // Heap allocation with ownership
    let string1 = String::from("Hello, world!");
    let string2 = string1; // Ownership is transferred (moved) to `string2`.
    
    println!("string2 = {}", string2);

    // Trying to use `string1` here would cause a compile error because its
    // ownership has moved to `string2`.
    // Uncommenting the line below will fail to compile:
    // println!("string1 = {}", string1);
    
    // Demonstrating borrowing instead of ownership transfer
    let string3 = String::from("Ownership stays in main()");
    print_string(&string3);  // Pass a reference (borrow)
    println!("string3 still accessible here: {}", string3);

    // Demonstrating mutable borrowing
    let mut my_string = String::from("Hello");
    append_text(&mut my_string);
    println!("After mutation: {}", my_string);
}

// A function that borrows a string immutably
fn print_string(s: &String) {
    println!("print_string sees: {}", s);
}

// A function that borrows a string mutably and changes it
fn append_text(s: &mut String) {
    s.push_str(", world!");
}
