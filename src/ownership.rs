fn main() {
    
    let s3 = String::from("Hello, world!");
    {
        let s4 = s3; // s3 ownership is moved to s4
        // println!("Outside inner scope: {}", s3); // This line would cause a compile error
        // because s3 is no longer valid after being moved to s4.
        println!("Inside inner scope: {}", s4);
    }
    // println!("After inner scope: {}", s3); // This line would also cause a compile error
}