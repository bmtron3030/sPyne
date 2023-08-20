# sPyne

sPyne is a Rust-built API that offers a range of functions for running one or multiple Python files as "routines". These functions provide a convenient way to incorporate Python functionality in the form of a sturdy backbone, or _spine_, with the help of Rust. This combination enables applications to harness the power of both both languages within a single project, combining the performance and safety of Rust with the extensive libraries and ecosystem of Python.

### Requirements
This file requires that the latest version of ````rustup```` is downloaded globally onto your device. To download, visit https://www.rust-lang.org/tools/install.

### Installation
1. Make sure the latest version of ````rustup```` is downloaded globally onto your device. (To check if it is downloaded, type: ````rustup --version```` for the current version installed on your device)
2. Download the ````spyne```` folder
3. Save it into any given folder directory
4. Open a terminal window
5. Navigate to the ````spyne```` folder directory with the terminal window
6. Run ````cargo build```` to build the project
7. Run ````cargo run```` to verify the installation was successful. A message will be printed if no errors occured.

### Usage
1. Make sure the ````spyne.rs```` file is in the same directory as the Rust file you are trying to call it from
2. Paste the ````use mod spyne;```` line to import it into any Rust file
3. Call any function from the ````spyne.rs```` file with ````spyne::---````

### Limitations
- Functions do not have any return types, they strictly execute any given Python file or files.
- Arguments need to be passed in the form of an array of strings, even if only one arg plans to be used.
- Python files need to convert the given arguments from strings into their desired data types.
- Routine sets are unable to have their order altered once execution starts.

### Examples
1. The following snippet runs a routine that prints "Hello World!".

(_All required files for this example can be found in the "/src/example_one/" folder of the /spyne/ project_)
````
mod spyne;

fn main() {
    //Replace the following line with the desired file path in the form of a string to the hello_wolrd.py file
    let file_string: String = String::from(".../spyne/src/example_one/hello_world.py"); 

    spyne::run_routine_no_args(&file_string, true);
}
````


2. The following snippet runs a routine that takes in two seperate numbers as input and prints whether one of them is greater/less than the other or if they equal eachother.

(_All required files for this example can be found in the "/src/example_two/" folder of the /spyne/ project_)
````
mod spyne;

fn main() {
    //Replace the following line with the desired file path in the form of a string to the number_compare.py file
    let file_string: String = String::from(".../spyne/src/example_two/number_compare.py");

    let input_number_one:i32 = 15;
    let input_number_two:i32 = 10;
    let input_args = vec![input_number_one.to_string(), input_number_two.to_string()];

    spyne::run_routine_with_args(&file_string, &input_args, true);
}
````


3. The following snippet runs a set of routines that print out "Hello World!" in six different languages.

(_All required files for this example can be found in the "/src/example_three/" folder of the /spyne/ project_)
````
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
````


4. The following code snippet calls the same routine multiple times but gives a different amount of arguments for every time it is called. The routine that is called prints out all of the arguments given to it.

(_All required files for this example can be found in the "/src/example_four/" folder of the /spyne/ project_)
````
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
````


5. The following code snippet runs one routine in a for loop for a certain amount of iterations based on a numeric value. The routine that is called will generate two random numbers and compare their values.

(_All required files for this example can be found in the "/src/example_five/" folder of the /spyne/ project_)
````
mod spyne;

fn main() {
    //Replace the following line with the desired file path in the form of a string to the random_number_generate_and_compare.py file
    let file_string: String = String::from(".../spyne/src/example_five/random_number_generate_and_compare.py");
    let loop_max:i32 = 3;    

    spyne::run_routine_in_for_loop_no_args(&file_string, true, loop_max);
}
````

### Future/Planned Updates To Come
- _As-Is Argument Passing_: Pass in a vector of arguments of ANY type other than strings so no data conversion is required.
- _Function Return Values_: Functions will be able to return values for more programability within Rust projects.
- _async/.await Functionality_: Routines will be able to run standard or as threads with the built-in Rust _await_ and _.async_ functionality.
- _Dynamic Routine Reordering_: Allow routine sets to have their routines reordered after X amount of them have been executed already.
