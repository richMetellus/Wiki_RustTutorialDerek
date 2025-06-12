#![allow(unused_variables)]
fn main() {

    const PI: f32 = 3.141592;

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
    let generic_rec = GenericRectangle { length: 4, height: 10.5} ;
    println!("GenericRectangle length : {}, height: {}", generic_rec.length, generic_rec.height);

     // We can tie struct properties to functions using Traits
    // You can create functions that can be used by any structs
    // that implement the right traits. This is defining sort of like an interface 
    // in object-oriented programming. It is defining the things anyone who 
    // implements our shape must do.
    trait Shape {
        // This is a constructor which returns itself i.e a Shape
        fn new(length: f32, width: f32) -> Self;

        // An area function that belongs to this trait. It take something that is 
        // a shape type of trait and is going to return a float.
        fn area(&self) -> f32;
    }

    // Define rectangle and circle struct
    struct Rectangle {length: f32, width: f32}
    struct Circle {length: f32, width: f32}

    // Implement the trait for rectangle
    impl Shape for Rectangle{
        // Constructor
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }

        // self allows us to refer to parameters for this struct
        fn area(&self) -> f32{
            return self.length * self.width;
        }
    }

    // Implement the trait for circle
    impl Shape for Circle{
        // Constructor
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }

        fn area(&self) -> f32{
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    // Create circle and rectangle with Shape
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("non generic Rectangle length : {}, width: {}", rec.length, rec.width);
    println!("Rectangle Area : {}", rec.area());
    println!("non generic circle length (i.e diameter) : {}, width (not used): {}", rec.length, rec.width);
    println!("Circle Area : {}", circ.area());

    // We can implement methods on structs using generics
    // impl<T, U>Shape<T, U> ...


}
