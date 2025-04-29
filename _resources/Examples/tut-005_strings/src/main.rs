fn main() {
    println!("-------------------------------------------------------------------");

    // ----- STRINGS -----
    // There are 2 types of strings
    // 1. String : Vector of bytes that can be changed
    // 2. &str : string slice, reference that Points to the string and allows for viewing


    // Create an empty growable string
    let mut st1 = String::new(); // String type

    // Insert a character at the end of a string
    st1.push('A');

    // Insert a string at the end
    st1.push_str(" word"); // notice the space

    // Iterate through words by splitting at whitespace
    println!("Splitting st1 = \"{st1}\" - using whitespace delimiter");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    // Replace "A" from st1 to Another and bind the value to st2.
    println!("String substitution:  Replacing 'A' with \"Another\" in st1 = \"{st1}\" ");
    let st2 = st1.replace("A", "Another");
    println!("st2 = \"{}\"", st2);
    println!("is st1 still in scope? Yes; st1 ={st1}");
    println!("-------------------------------------------------------------------");
    
    // Create string of characters
    let st3 = String::from("x r t b h k k a m c"); // just throw random char separated by space
    
    println!("string to start with: st3={}", st3);

    // Convert to a vector (Vector are covered in a different chapter)
    println!("Converting st3 to vector");
    let mut v1: Vec<char> = st3.chars().collect();

    // Sort characters
    println!("Sorting the vector collection");
    v1.sort();

    // TODO: Uncomment this to see compiler error:
    // for x in v1 {
    //     println!("{}", x);
    // }

    // Remove duplicates (so no K K)
    print!("\n--------\n");
    println!("Remove the duplicates in the  vector collection");
    v1.dedup();

    // Cycle through vector
    for char in v1 {
        print!("{},", char);
    }

    print!("\n--------\n");

    // Create a string literal
    let st4: &str = "Random string";

    // Convert to heap allocated String
    let mut st5: String = st4.to_string();
    println!("st5={}", st5);

    // Convert string into an array of bytes
    let _byte_arr1 = st5.as_bytes();

    // Get a slice of a string from index 0 to 5
    let st6: &str = &st5[0..6];
    println!("st6 as string slice: &st5[0..6]={}", st6);

    // Get length of string
    println!("st6 String Length : {}", st6.len());

    // Delete values in a string if mutable
    st5.clear();

    // Combine strings
    let st6 = String::from("Just some");
    let st7 = String::from("words");
 
    // st6 can no longer be used
    // You can only add a reference to a string to another
    let st8 = st6 + &st7; // st7 still exist and not dropped
    // println!("st6 as string: {st6}"); // Illegal borrow of moved value 

    println!("st8 as String st8={st8}\nst8 len={}", st8.len());

    // Cycle through letters in a string and print unicode
    println!("Dumping st8 as bytes (unicode)");
    for char in st8.bytes() {
        print!("{},", char);
    }
    
    println!("\n--------");
    // Cycle through letters in a string and print characters
    println!("Dumping st8 as chars");
    for char in st8.chars() {
        print!("{},", char);
    }

    println!("\n-------------------------------------------------------------------");

}
