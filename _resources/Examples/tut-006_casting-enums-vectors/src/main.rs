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


}
