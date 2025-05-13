// ----- FUNCTIONS -----
// You can define functions before or after main
// function definition
fn say_hello(){
    println!("Hello");
}

// You can pass arguments to functions
fn get_sum(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x+y);
}

// Return a value
fn get_sum_2(x: i32, y: i32) -> i32 {
    // This expression is returned
    // If you used a semicolon you'd get an error because
    // a statement don't evaluate to a value
    x + y
}

// You can also use return
fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

// Return multiple values
fn get_2(x: i32) -> (i32, i32){
    return (x+1, x+2);
}

// This function sums values in a list (Receives reference to list)
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

// We get 2 generic types of the same type and return that same type
fn get_sum_gen<T>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    // ----- FUNCTIONS calls -----
    say_hello();
    println!("--function performing simple addition--");
    get_sum(4, 5);
    println!("{} + {} = {}", 5, 3, get_sum_2(5, 3));
    println!("{} + {} = {}", 7, 8, get_sum_3(7, 8));

    // Get multiple values
    let (val_1, val_2) = get_2(3);
    println!("get_2: Function returns tuple; f(x)= (x+1, x+2), where x=3 ({}, {})", val_1, val_2);

    let num_list = vec![1,2,3,4,5];
    println!("Given a list [1,2,3,4,5] \nSum of list = {}", sum_list(&num_list));

    // ----- GENERIC TYPES -----
    // We can specify the data type to be used at a later time with generics
    // It is mainly used when we want to create functions that can work with
    // multiple data types. It is used with structs, enums, traits, etc.
    // which we'll talk about later
    println!("5 + 4 = {}", get_sum_gen(5,4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));
}
