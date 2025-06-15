// Used for working with files
// You could also use nested paths like this
use std::io::{Write, BufReader, BufRead};
use std::fs::File;

fn main() {
    // ----- READING & WRITING TO FILES & ERROR HANDLING -----
    // Rust doesn't have exceptions like other languages. It handles
    // recoverable errors with Result and the panic! macro for
    // unrecoverable errors

    // When the panic! macro executes your program prints an error
    // memory is cleaned up and the program quits
    // panic!("Terrible Error");

    // Accessing an index that doesn't exist calls panic
    // let lil_arr = [1,2];
    // println!("{}", lil_arr[10]);

    // File to create
    let path = "lines.txt";

    // Result has 2 variants Ok and Err
    // enum Result<T, E> {
    // Ok(T),
    // Err(E), }
    // Where T represents the data typeof the value returns and E
    // the type of error

    // Create file and handle errors with match
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };

    // Write to file and define the panic! error message with expect
    write!(output, "Just some\nRandom Words").expect("Failed to write to file");

    // Open the file and if everything is ok unwrap returns the file
    // and if not panic! triggers an error (You could replace unwrap with ?)
    let input = File::open(path).unwrap();
    // Read file using buffering
    let buffered = BufReader::new(input);

    // Cycle through and print the lines
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

}