fn main() {
    // Stack allocation
    let x = 5;
    let _y = x;  // Copy semantics for primitive types
    println!("{}", x);
    println!("{}", _y);
    
    // Heap allocation with ownership
    let string1 = String::from("hello world!"); 
    let _string2 = string1;  // Ownership transferred to string2
    println!("{}", _string2);
    println!("{}", string1);  // Would cause compile error
}
