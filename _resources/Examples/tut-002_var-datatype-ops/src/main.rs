// constant variables demo code:

fn main() {
    println!("-------------------------------------------------------------------");

    const ONE_MIL: u32 = 1_000_000; // need explicit type annotation
    const PI : f32 = 3.141592;
    let age = "47"; // a string, implicitly typed. defined with double quotes
    let mut age: u32 = age.trim().parse() // LEGAL in rust to re-use the same variable name. Shadowing
        .expect("Age wasn't assigned a number"); 

    // now that the age is a u32, we can do operation.
    age = age + 1; // increment by 1.

    println!("I'm {} and I want ${} so I can buy some {}", age, ONE_MIL, PI);

    println!("-------------------------------------------------------------------");
    
    // Integer Datatypes and their maximum value:
    println!("Max i8: {}", i8::MAX);
    println!("Max u8: {}", u8::MAX);
    println!("Max i16: {}", i16::MAX);
    println!("Max u16: {}", u16::MAX);
    println!("Max i32: {}", i32::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max i64: {}", i64::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max i128: {}", i128::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max isize: {}", isize::MAX);
    println!("Max usize: {}", usize::MAX);

    // Some float datatype and their max
    println!("-------------------------------------------------------------------");
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
    println!("-------------------------------------------------------------------");

    // Booleans can have for value: true or false 
    let _unused_bool_as_place_holder = true; // start with _ so compiler ignore
    let _another_bool: bool = false; // explicit type annotation 
    
    
    // characters are defined using single quote ''
    let _my_grade = 'A'; // implicitly infer by compiler to be char type
    let _min_grade: char = 'F'; // with explicit type annotation
    let mut unassigned_char: char; // var definition, need explicit type
    unassigned_char = '🫂'; // character is 4 bytes in Rust, encoded in Unicode Scalar Value.
    println!("people hugging emoji: {}", unassigned_char);
    unassigned_char = '🍎';
    println!("red apple emoji: {}", unassigned_char);
    
}
