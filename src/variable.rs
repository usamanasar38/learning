fn main() {
    let str = String::from("Hello, world!");

    let chr = str.chars().nth(0).unwrap();
    println!("The first character is: {}", chr);
}
