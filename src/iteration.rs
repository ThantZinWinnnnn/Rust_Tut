pub fn loop_example(x : i32) -> (){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * x;
        }
    };
    println!("Result: {}", result);
}


pub fn labeled_loop ()-> () {
    let mut count = 0;
    //must write ' before the name of the label
    'outer_loop: loop {
        println!("Outer Loop: {}", count);
        let mut remaining = 10;
        
        loop {
            println!("Inner Loop: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

pub fn text_while_loop () -> () {
    let mut number = 5;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

pub fn for_loop () -> () {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
}