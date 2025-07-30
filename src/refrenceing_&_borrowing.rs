// Immutable references allow you to read the value they point to
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable references allow you to modify the value they point to
fn change(s: &mut String) {
    s.push_str(", world!"); // This modifies the string in place
}

fn main() {
    let s1 = String::from("Hello, world!");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


    // Mutable reference example
    let s2 = String::from("Hello");
    change(&mut s2); // s2 is mutable, so we can change it
    println!("After change: {}", s2); // Output: After change: Hello, world!
}