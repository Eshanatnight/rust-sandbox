pub fn run()
{
    let x = vec![1, 2, 3, 4, 5];
    let res = accumulate(x);
    println!("{}", res);
}

fn accumulate(vec: Vec<i32>) -> i32
{
    let mut sum = 0;
    for i in vec
    {
        sum += i;
    }

    return sum;
}