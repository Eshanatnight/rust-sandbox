pub fn run()
{
    println!("\nRust - Data Types");
    println!("Integer Types");
    let int: i8 = -1; // i8 is an 8-bit signed integer
    let uint: u8 = 1; // u8 is an 8-bit unsigned integer
    let int: i16 = -1; // i16 is a 16-bit signed integer
    let uint: u16 = 1; // u16 is a 16-bit unsigned integer
    let int: i32 = -1; // i32 is a 32-bit signed integer
    let uint: u32 = 1; // u32 is a 32-bit unsigned integer
    let int: i64 = -1; // i64 is a 64-bit signed integer
    let uint: u64 = 1; // u64 is a 64-bit unsigned integer

    println!("\nFloating Point Types");
    let float: f32 = 1.0; // f32 is a 32-bit floating point number
    let double: f64 = 1.0; // f64 is a 64-bit floating point number

    println!("\nBoolean Types");
    let boolean: bool = true; // bool is a boolean type

    println!("\nCharacter Types");
    let chara: char = 'a'; // char is a unicode character

    println!("\nString Types");
    let string: String = "Hello".to_string(); // String is a string type
    let string: &str = "Hello"; // &str is a string type
    let mut str: String = String::new();
    str = "Hello".to_string();
    println!("{}", str);

    println!("\nTuple Types");
    let one_tuple:(i32, i32, i32) = (1, 2, 3); // Tuple with 3 elements of i32 type
    let tuple: (i32, f64, u8) = (500, 6.4, 1);  // Tuple with 3 elements of different types
    let new_tuple: (i32, f64, &str) = (500, 6.4, "Hello"); // Tuple with 3 elements of different types
    let next_tuples: (i32, f64, String) = (500, 6.4, "Hello".to_string()); // Tuple with 3 elements of different types

    println!("{:?}\n{:?}\n{:?}", one_tuple, tuple, next_tuples);

    println!("\nArray Types");
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // Array with 5 elements of i32 type

    println!("\nVec Types");
    let mut vec: Vec<i32> = Vec::new(); // Vec is a vector type
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(88);

    // sort_by takes a lambda function as an argument
    vec.sort_by(|a, b| b.cmp(a));

    println!("{:?}", vec);

    println!("\nHash Map Types");
    let mut map: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    print!("{:?}", map);

    println!("\nEnum Types");
    #[derive(Debug)]
    enum Color
    {
        Red(i32),
        Green(i32),
        Blue(i32)
    }

    let color: Color = Color::Red(255);
    let color: Color = Color::Green(255);
    //let color: Color = Color::Blue(255);

    println!("{:?}", color);
}