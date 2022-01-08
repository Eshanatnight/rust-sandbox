pub fn run()
{
    // Infinite loop probably not a good idea
    let mut counter = 0;
    println!("Infinite loop");
    loop
    {
        println!("Counter: {}", counter);
        counter += 1;
        if counter == 10
        {
            break;
        }
    }

    // Range for loop
    for x in 0..10
    {
        println!("Counter: {}", x);
    }

}