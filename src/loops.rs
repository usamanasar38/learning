pub fn main() {
    let str = String::from("Usama Nasar");
    println!("First name {}", getFirstName(str))
    
}

pub fn getFirstName(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}