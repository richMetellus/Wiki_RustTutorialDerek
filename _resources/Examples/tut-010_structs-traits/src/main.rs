#![allow(unused_variables)]
fn main() {

    // ----- STRUCTS -----
    // A struct is a custom data type that stores multiple
    // types of data

    struct Customer{ 
        name: String, 
        address: String,
        balance: f32,
    }
    
    // Structs are primarily stored on the stack when used as local variables.
    // Create struct bob
    let mut bob = Customer { // locally defined inside of main, so it is part of stack data
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50
    };

    // say Bob moves and you want to update his address
    bob.address = String::from("505 Main St"); // since Bob is mutable we can update the struct fields.
    println!("Address : {}", bob.address);

    // You could accept multiple data types using generics like
    // we did with functions. If we had a GenericRectangle struct
    // that could accept floats or ints we would need 2 generics
    struct GenericRectangle<T, U> {
        length: T, 
        height: U,
    }

    // define a struct
    let rec = GenericRectangle { length: 4, height: 10.5} ;
    println!("GenericRectangle length : {}, height: {}", rec.length, rec.height);


}
