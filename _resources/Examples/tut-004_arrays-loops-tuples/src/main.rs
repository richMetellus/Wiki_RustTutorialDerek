use std::mem; 

fn main() {
    
    // ----- ARRAYS -----
    // Elements in an array must be of the same type
    // and have a fixed size
    const ARR_1: [i32; 4] = [1,2,3,4];
    let arr1_ref: &[i32] = &ARR_1;
    // Get value by index starting at 0
    println!("1st : {}", ARR_1[0]);
    println!("2nd element : {}", ARR_1[1]);

    // Get array length
    println!("Length : {}", ARR_1.len());
    
    // Get the size (bytes) of the array.. arrays are continuous block of memory
    println!("total size of memory occupied by 1 i32 : {} bytes", mem::size_of::<i32>());
    println!("total size of memory occupied by ARR_1 : {} bytes", mem::size_of_val(&ARR_1));
    println!("total size of memory occupied by ARR_1 using arr1_ref: {} bytes", mem::size_of_val(arr1_ref));
    println!("-------------------------------------------------------------------");

     // ----- LOOP -----
    // Create an infinite loop that ends when break is called
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    loop {
        // check if number is even
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue; // Goes to next iteration of loop i.e skip
        }

        if arr_2[loop_idx] == 9 {
            break; // Breaks out of loop
        }

        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
    
    println!("------------------------------- while loop-------------------------");

    // ----- WHILE LOOP -----
    // Looping based on a condition
    loop_idx = 0; // reset our index
    while loop_idx < arr_2.len(){
        println!("Arr[{}] : {}", loop_idx, arr_2[loop_idx]);
        loop_idx += 1;
    }

    println!("------------------------------- for loop----------------------------");

    // ``for`` loops works better for cycling through collections
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }

    println!("------------------------------- Tuples ----------------------------");

    // ----- TUPLES -----
    // Tuples can contain multiple data types in a list of fixed size
    // We convert to strings with to_string()
    let my_tuple: (u8, String, f64, f32) = (25, "Ricky".to_string(), 280_000.14, 2.00);

    // You can get values by index starting at 0
    println!("2nd element in tuple: Name = {}", my_tuple.1);

    // You can assign values to multiple variables
    //unused variables are prefixed with _
    let (v1, v2, v3, _v4) = my_tuple;
    println!("Age : {},\nName: {},\nnet worth: {}", v1, v2, v3);
    
    /* The below code won't compile because of the string that is part of the 
    tuple. Uncomment it to see more:

    Compiler message: "due to the 
    move occurs because `my_tuple.1` has type `String`, which does not implement the `Copy` trait"  

    More on strings later
    */

    // let (age,  name, _net_worth, _mult) = my_tuple;
    // println!("Name: {}, {}", name, age);

}
