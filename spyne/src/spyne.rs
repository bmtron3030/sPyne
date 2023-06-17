use std::path::Path;
use std::process::Command;
use std::process::Output;
use std::thread;

//Function that checks if a file exists regardless of file extension
//Returns true if the file exists, false otherwise
//Used for internal use, can be made public for external use.
fn file_exists(file_path: &str) -> bool{
    let path = Path::new(file_path);
    path.exists() && path.is_file()  
}

//Function that checks if any given file is a Python file specifically
//by identifying its file extension(".py" or not ".py")
//Returns true if the file is a Python file, false otherwise
//Used for internal use, can be made public for external use.
fn is_python_file(file_path: &str) -> bool {
    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str);
    if let Some(ext) = extension {
        ext == "py"
    } else {
        false
    }
}

//Function that prints the output of a single routine
//Doesnt return anything, only prints the output or if an error occured
//with attempting to run the routine
//Used for internal use, can be made public for external use.
fn print_routine_output(result: Result<Output, std::io::Error>) {
    match result {
        Ok(result) => {
            if !result.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&result.stdout));
            } else if !result.stderr.is_empty() {
                println!("Error occured while executing routine -> {}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(error) => {
            println!("Error occured while executing routine -> {}", error);
        }
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)] 

//Function that runs a Python file as a "routine" without any arguments or "args".
//Simultanously checks if the given file exists AND if it is a Python file BEFORE exceuting it as 
//a "routine" without any given arguments or "args". Takes the file path as an 
//argument in the form of a string and a boolean to indicate if
//it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_no_args(file_path: &String, verbose: bool){
    let file_exists_bool: bool = file_exists(&file_path);
    let is_python_file_bool: bool = is_python_file(&file_path);
    if file_exists_bool && is_python_file_bool{
        let routine_output = Command::new("python").arg(file_path).output();
        if verbose == true {
                print!("Running {} ... \n", file_path);
                let _ = print_routine_output(routine_output);
        }
    } else if is_python_file_bool == false {
            print!("{} is not a valid Python file. Please use the directory of a valid Python file.",file_path);
    } else if file_exists_bool == false {
            print!("{} does not exist as a file. Please use the directory of an existing Python file.",file_path);
    } else { 
            print!("{} does not exist as a file. Please use the directory of an existing, valid Python file.",file_path);
    }

}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs multiple routines in the order they are given in.
//Simultanously checks if ALL given file exists AND if it is a Python file BEFORE exceuting it as
//a "routine" without any given arguments or "args". Takes the set of file paths as an argument in
//the form of a vector of strings and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_set_no_args(file_paths: &Vec<String>, verbose: bool){
    file_paths
    .iter()
    .for_each(|file_path| run_routine_no_args(file_path, verbose));
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a Python file as a "routine" in the form of a Rust thread
//Simultanously checks if the given file exists AND if it is a Python file BEFORE transferring
//any given file to a thread. Takes the file path as an argument in the form of a string 
//and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_thread_no_args(file_path: &String, verbose: bool){
    let file_exists_bool: bool = file_exists(&file_path);
    let is_python_file_bool: bool = is_python_file(&file_path);
    if file_exists_bool && is_python_file_bool{
        let file_clone = file_path.to_owned();
        let routine_thread_handle: thread::JoinHandle<(String, Result<Output, std::io::Error>)> = thread::spawn(move || {
            let routine_output = Command::new("python").arg("-c").arg(&file_clone).output();
            (file_clone, routine_output)
        });
        let (_, routine_thread_output) = routine_thread_handle.join().unwrap();

        if verbose == true {
                print!("Running {} ... \n", &file_path);
                let _ = print_routine_output(routine_thread_output);
        }
    } else if is_python_file_bool == false {
            print!("{} is not a valid Python file. Please use the directory of a valid Python file.",file_path);
    } else if file_exists_bool == false {
            print!("{} does not exist as a file. Please use the directory of an existing Python file.",file_path);
    } else { 
            print!("{} does not exist as a file. Please use the directory of an existing, valid Python file.",file_path);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs multiple routines in the form of a set of Rust threads created in the form that they are given in. 
//Takes the set of file paths as an argument in the form of a vector of strings and
//a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_thread_set_no_args(file_paths: &Vec<String>, verbose: bool) {
    let mut threads:Vec<thread::JoinHandle<(String, ())>>  = vec![];
    for file in file_paths{
        let file_clone = file.to_owned();
        let routine_thread_handle: thread::JoinHandle<(String, ())> = thread::spawn(move || {
            let routine_result = run_routine_no_args(&file_clone, verbose);
            (file_clone, routine_result)
        });

        threads.push(routine_thread_handle);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a single routine in a loop for any given number of times.
//Takes the file path as an argument in the form of a string, an i32 for the number of times to
//run the routine for, and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_in_for_loop_no_args(file_path: &String, verbose: bool, loop_range:i32){
    for _i in 0..loop_range {
        run_routine_no_args(file_path, verbose);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a routine set in a loop for any given number of times in the SAME order originally given in. 
//Takes multiple file paths as an argument in the form of a vector of strings for 
//each Python file to run as a routine, an i32 for the number of times to run the routine set for, and
//a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_set_in_for_loop_no_args(file_paths: &Vec<String>, verbose: bool, loop_range:i32){
    for _i in 0..loop_range {
        run_routine_set_no_args(file_paths, verbose);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a single Python file in the form of a routine with any given amount of arguments or "args"
//Takes the file path as an argument in the form of a string, the arguments as a vector of strings (any length), 
//and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_with_args(file_path: &String, file_args:&Vec<String>, verbose: bool){
    let file_exists_bool: bool = file_exists(&file_path);
    let is_python_file_bool: bool = is_python_file(&file_path);
    if file_exists_bool && is_python_file_bool{
        let routine_output = Command::new("python").arg(file_path).args(file_args).output();
        if verbose == true {
                print!("Running {} ... \n", file_path);
                let _ = print_routine_output(routine_output);
        }
    } else if is_python_file_bool == false {
            print!("{} is not a valid Python file. Please use the directory of a valid Python file.",file_path);
    } else if file_exists_bool == false {
            print!("{} does not exist as a file. Please use the directory of an existing Python file.",file_path);
    } else { 
            print!("{} does not exist as a file. Please use the directory of an existing, valid Python file.",file_path);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function used to run a multiple Python files in the form of a routine set with any given amount of arguments or "args" for each file.
//Takes multiple file paths as an argument in the form of a vector of strings, a vector of vectors of strings that represent the arguments for each file,
//and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).

//This function WILL NOT WORK if the vector of file paths is not the same length as the vector of arguments, and vice versa. There
//is a built in safeguard that addresses this, but it is uttely important to outline this 
//functionality for safety. That being said, there is no limit to how many arguments can be passed through to 
//each routine as per the standard functionality of the run_routine_with_args funciton.
//Does not return anything.
pub fn run_routine_set_with_args(file_paths: &Vec<String>, file_args:&[Vec<String>], verbose: bool){
    if file_paths.len() != file_args.len() {
        println!("Error: Vectors have different lengths. Terminating function.");
        return;
    } else {
        for i in 0..file_paths.len() {
            run_routine_with_args(&file_paths[i], &file_args[i], verbose);
        }
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a single routine with parameters in a loop for any given number of times.
//Takes the file path as an argument in the form of a string, an i32 for the number of times to
//run the routine for, and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_in_for_loop_with_args(file_path: &String, file_args:&Vec<String>, verbose: bool, loop_range:i32){
    for _i in 0..loop_range {
        run_routine_with_args(file_path, file_args, verbose);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a routine set with parameters in a loop for any given number of times in the SAME order originally given in. 
//Takes multiple file paths as an argument in the form of a vector of strings for each Python file to run as a routine, 
//a vector of vectors of strings that represent the arguments for each file, an i32 for the number of times to run the 
//routine set for, and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_set_in_for_loop_with_args(file_paths: &Vec<String>, file_args:&[Vec<String>], verbose: bool, loop_range:i32){
    for _i in 0..loop_range {
        run_routine_set_with_args(file_paths, file_args, verbose);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs a Python file as a "routine" in the form of a Rust thread
//Simultanously checks if the given file exists AND if it is a Python file BEFORE transferring
//any given file to a thread. Takes the file path as an argument in the form of a string 
//and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_thread_with_args(file_path: &str, file_args: &[String], verbose: bool) {
    let file_exists_bool: bool = file_exists(file_path);
    let is_python_file_bool: bool = is_python_file(file_path);
    if file_exists_bool && is_python_file_bool {
        let file_clone = Path::new(&file_path).to_string_lossy().into_owned();
        let file_args_clone = file_args.to_vec();
        let routine_thread_handle: thread::JoinHandle<(String, Result<Output, std::io::Error>)> =
            thread::spawn(move || {
                let routine_output = Command::new("python")
                    .arg("-c")
                    .arg(&format!("exec(open('{}').read())", &file_clone))
                    .args(&file_args_clone)
                    .output();
                (file_clone, routine_output)
            });
        let (_, routine_thread_output) = routine_thread_handle.join().unwrap();
        if verbose {
            print!("Running {} ... \n", file_path);
            print_routine_output(routine_thread_output);
        }
    } else if is_python_file_bool == false {
            print!("{} is not a valid Python file. Please use the directory of a valid Python file.",file_path);
    } else if file_exists_bool == false {
            print!("{} does not exist as a file. Please use the directory of an existing Python file.",file_path);
    } else { 
            print!("{} does not exist as a file. Please use the directory of an existing, valid Python file.",file_path);
    }
}

//This following line is present to supress the unused code warning from the cargo compiler.
//Only uncomment for debugging purposes.
#[allow(dead_code)]

//Function that runs multiple routines in the form of a set of Rust threads created in the form that they are given in with their given arguments.
//Takes the set of file paths as an argument in the form of a vector of strings, a vector of vectors of strings that represent the arguments
//for each file and a boolean to indicate if it should be verbose or not (true for verbose, false otherwise).
//Does not return anything.
pub fn run_routine_thread_set_with_args(
    file_paths: Vec<String>,
    file_args: &[Vec<String>],
    verbose: bool,
) {
    if file_paths.len() != file_args.len() {
        println!("Number of file paths and arguments must match.");
        return;
    }
    let routine_threads: Vec<thread::JoinHandle<(String, Result<Output, std::io::Error>)>> =
        file_paths
            .iter()
            .map(|file_path| file_path.to_owned())
            .zip(file_args)
            .map(|(file_path, args)| {
                let file_clone = Path::new(&file_path).to_string_lossy().into_owned();
                let args_clone = args.to_owned();
                thread::spawn(move || {
                    let routine_output = Command::new("python")
                    .arg("-c")
                    .arg(&format!("exec(open('{}').read())", &file_clone))
                    .args(args_clone)
                    .output();
                    (file_clone, routine_output)
                })
            })
            .collect();
    for thread_handle in routine_threads {
        let (file_path, routine_thread_output) = thread_handle.join().unwrap();
        if verbose {
            println!("Running {} ...", file_path);
            print_routine_output(routine_thread_output);
        }
    }
}
