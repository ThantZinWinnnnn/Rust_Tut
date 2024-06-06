pub mod functions;
mod conditions;
mod iteration;
pub mod reference;
pub mod string;
pub mod struct_cp;
pub mod method;
pub mod enum_cp;
mod vector;

mod garden;

fn main() {
    let result = functions::add(5, 5);
    let sub_result = functions::subtract(5, 4);
    conditions::condition(6);
    // iteration::loop_example(10);
    // iteration::labeled_loop();
    // iteration::text_while_loop();
    // iteration::for_loop();
    string::string_example();
    garden::vegetables::hello();
    println!("Result: {}, Subtract Result: {}", result, sub_result);
}
