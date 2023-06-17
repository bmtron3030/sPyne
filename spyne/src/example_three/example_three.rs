//To run this example: 
//1.Copy this entire file into the src folder of this Rust project
//2.Rename this file to "main.rs" 
//3.Run "cargo build"
//4.Run "cargo run"
mod spyne;

fn main() {
    //Replace the following lines with the desired file paths in the form of 
    //a string to each of the different hello_wolrd.py language files
    let file_strings: Vec<String> = vec![
        String::from("C:.../spyne/src/example_three/hello_world_english.py"),
        String::from("C:.../spyne/src/example_three/hello_world_german.py"),
        String::from("C:.../spyne/src/example_three/hello_world_greek.py"),
        String::from("C:.../spyne/src/example_three/hello_world_japanese.py"),
        String::from("C:.../spyne/src/example_three/hello_world_korean.py"),
        String::from("C:.../spyne/src/example_three/hello_world_spanish.py")
    ];

    spyne::run_routine_set_no_args(&file_strings, true);
}