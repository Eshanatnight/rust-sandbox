pub fn run()
{
    // STDOUT print
    println!("Hello From the print.rs File");

    // Formatting Basic
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Mass");

    // Positional Parameters
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);
}