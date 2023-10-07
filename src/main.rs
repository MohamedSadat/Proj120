//disable warning
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
//disable camel case warning
#![allow(non_snake_case)]
use std::thread::sleep;
use std::time::Duration;
mod cglib;
mod cgstring;
//import cglib
//use cglib;

fn main() {
    println!("Hello, world!");
    let result = cglib::divide(10.0, 2.0);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }
    //read value from user
    println!("Enter a number to multiply: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: f64 = input.trim().parse().unwrap();
    println!("You entered {}", input);
    let result =cglib::multiply(input);
    println!("Result: {}", result);

    //check grade
    println!("Enter a grade: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: f64 = input.trim().parse().unwrap();
    println!("You entered {}", input);
    let result =cglib::check_grade(input);
    
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: grade is more than 10"),
    }
    //reverse string
    
let mut res =cgstring::reverse_string("hello");
print!("reverse string: {}",res);
    sleep(Duration::from_secs(5));
}


