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


}
