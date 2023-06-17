//To run this example: 
//1.Copy this entire file into the src folder of this Rust project
//2.Rename this file to "main.rs" 
//3.Run "cargo build"
//4.Run "cargo run"
mod spyne;

fn main() {
    //Replace the following lines with the desired file paths in the form of a string to the args_printer.py language files
    let file_strings: Vec<String> = vec![
        String::from(".../spyne/src/example_four/args_printer.py"),
        String::from(".../spyne/src/example_four/args_printer.py"),
        String::from(".../spyne/src/example_four/args_printer.py"),
        String::from(".../spyne/src/example_four/args_printer.py"),
        String::from(".../spyne/src/example_four/args_printer.py"),
    ];

    let input_args: Vec<Vec<String>> = vec![
        vec![String::from("args1")],
        vec![String::from("args1"),String::from("args2")],
        vec![String::from("args1"),String::from("args2"),String::from("args3")],
        vec![String::from("args1"),String::from("args2"),String::from("args3"),String::from("args4")],
        vec![String::from("args1"), String::from("args2"),String::from("args3"),String::from("args4"),String::from("args5")]
    ];

    spyne::run_routine_set_with_args(&file_strings, &input_args,true);
}