pub fn run()
{
    // normal if statement
    let age = 19;

    if age > 16
    {
        println!("You can drive");
    }
    if age > 18
    {
        println!("You can vote!");
    }
    else
    {
        println!("You can't vote!");
    }

    // Rust does not have a ternary operator, but you can use if statements instead
    let is_of_age = if age >= 18 { true } else { false };

    println!("Is of age: {}", is_of_age);

    // if let statement
    // initializing a variable inside the if condition
    let favorite_color: Option<&str> = Some("Pink");

    if let Some(color) = favorite_color
    {
        println!("Favorite color: {}", color);
    }
    else
    {
        println!("No favorite color");
    }

    // Switch statement
    let num = Some(4);

    match num
    {
        Some(4) => println!("four"),
        Some(n) => println!("{}", n),
        None => println!("none"),
    }
}