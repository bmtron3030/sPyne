//To run this example: 
//1.Copy this entire file into the src folder of this Rust project
//2.Rename this file to "main.rs" 
//3.Run "cargo build"
//4.Run "cargo run"
mod spyne;

fn main() {
    //Replace the following line with the desired file path in the form of a string to the hello_wolrd.py file
    let file_string: String = String::from(".../spyne/src/example_one/hello_world.py"); 

    spyne::run_routine_no_args(&file_string, true);
}