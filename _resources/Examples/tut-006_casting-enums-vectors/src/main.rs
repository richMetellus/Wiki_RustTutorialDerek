use std::mem;

fn main() {
    println!("-------------------------------------------------------------------");

    // ----- CASTING WITH AS -----
    // You can convert to different types in multiple ways
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    // Cast using as unsigned int 32
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("int3_u32={}, size of {} bytes", int3_u32, mem::size_of_val(&int3_u32));
    println!("-------------------------------------------------------------------");

    // ----- ENUMS -----
    // Enums allow for the definition of custom data types

    // Create an Enum for days of week
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    // Define function for Day enum
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            } // <--- notice there is no `;` here
        }
    }

    // Use enum to store todays day
    let today: Day = Day::Monday;
    let yesterday:Day = Day::Sunday;

    // Perform different actions based on day
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend!!!"),
        Day::Sunday => println!("Weekend!!!"),
    }

    // Check if today is a weekend
    println!("Is today, Monday, the weekend? {}", today.is_weekend());
    println!("Is yesterday the weekend? {}", yesterday.is_weekend());
    println!("-------------------------------------------------------------------");

    // ----- VECTORS -----
    // Vectors are like arrays that can grow if mutable
    // They only store values of the same type

    // Create an empty vector with i32
    let _vec1: Vec<i32> = Vec::new();
    println!("empty vector created, but unused");

    // Create a vector with defined values
    let mut vec2 = vec![1, 2, 3, 4];

    println!("vector 2 created with values: vec2 = [1, 2, 3, 4]");

    // Add values to the end of a vector
    vec2.push(5);

    println!("value 5 is pushed to vec2\n --Getting element via index and `get`");

    // Get value by index
    println!("1st : {}", vec2[0]);

    // Verify value exists
    let _second_element: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    };

    // Cycle and change values
    println!("Doubling each value of element in mutable vector 2, then print");
    for i in &mut vec2 {
        *i *= 2;
    }

    // Cycle through vector values
    for i in &vec2 {
        println!("{}", i);
    }

    // Get number of values in a vector
    println!("Vec Length : {}", vec2.len());

    println!("Remove and return the last value in vec2");
    println!("Pop {:?}", vec2.pop());
    println!("Vec Length : {}", vec2.len());


}
