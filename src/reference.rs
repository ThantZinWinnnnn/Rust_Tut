pub fn reference_eg () -> () {
    let mut x = String::from("hello");
    change(&mut x);
    
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}