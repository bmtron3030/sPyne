//To run this example: 
//1.Copy this entire file into the src folder of this Rust project
//2.Rename this file to "main.rs" 
//3.Run "cargo build"
//4.Run "cargo run"
mod spyne;

fn main() {
    //Replace the following line with the desired file path in the form of a string to the number_compare.py file
    let file_string: String = String::from(".../spyne/src/example_two/number_compare.py");

    let input_number_one:i32 = 15;
    let input_number_two:i32 = 10;
    let input_args = vec![input_number_one.to_string(), input_number_two.to_string()];

    spyne::run_routine_with_args(&file_string, &input_args, true);
}