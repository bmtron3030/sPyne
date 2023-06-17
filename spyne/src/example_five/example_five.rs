//To run this example: 
//1.Copy this entire file into the src folder of this Rust project
//2.Rename this file to "main.rs" 
//3.Run "cargo build"
//4.Run "cargo run"
mod spyne;

fn main() {
    //Replace the following line with the desired file path in the form of a string to the random_number_generate_and_compare.py file
    let file_string: String = String::from(".../spyne/src/example_five/random_number_generate_and_compare.py");
    let loop_max:i32 = 3;    

    spyne::run_routine_in_for_loop_no_args(&file_string, true, loop_max);
}