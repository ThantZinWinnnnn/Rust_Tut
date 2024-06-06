pub fn condition(x:i32) -> i32 {
    if x > 0 {
        println!("{} is greater than 0", x);
    } else if x < 0 {
        println!("{} is less than 0", x);
    } else {
        println!("{} is equal to 0", x);
    }
    x
}
