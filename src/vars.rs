/*
    * Variables hold primitive data or references to data
    * Variables are immutable by default
*/

pub fn run()
{
    let name = "Kells";

    let mut age = 37;

    println!("My name is {}", name);
    println!("I am {}", age);

    age = 38;

    println!("My name is {}", name);
    println!("I am {}", age);

    // Define constant
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);

    println!("{} is {}", my_name, my_age);
}