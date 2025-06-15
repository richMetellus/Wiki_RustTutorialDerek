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

}