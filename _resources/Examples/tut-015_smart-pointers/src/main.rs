fn main() {
    // ----- SMART POINTERS -----
    // A pointer is an address to a location in memory. We have been
    // using them when we used the reference operator(&) to borrow
    // a value.

    // Strings and vectors are smart pointers. They own
    // data and also have functions for manipulating that data.

    // Smart pointers provide functionality beyond referencing locations
    // in memory. They can be used to track who has ownership of data.
    // Ownership is very important with Rust.

    // ----- BOX -----

    // The Box smart pointer stores data on the heap instead of the stack.
    // All values are stored on the stack by default

    // Stack : Stores values in a last in first out format
    // Data on the stack must have a defined fixed size

    // Heap : When putting data on the heap you request a certain
    // amount of space. The OS finds space available and returns
    // an address for that space called a pointer.

    // A Box is normally used when you have a large amount of data stored
    // on the heap and then you pass pointers to it on the stack.

    // Create a Box with value 10
    let b_int1 = Box::new(10);

    // Get the value
    println!("b_int1 = {}", b_int1);

    // If we try to create a Binary tree we get the error
    // the size for values of type `str` cannot be known at
    // compilation time within `TreeNode<T>`

    // This is saying we can't include nodes in a node because
    // the size of node depends on the size of multiple nodes
    // which confuses the compiler
    struct TreeNode<T> {
        pub left: TreeNode<T>,
        pub right: TreeNode<T>,
        pub key: T, // this caused error
    }

    // We have other problems in that Binary Trees eventually end
    // and Rust doesn't like Null values so we have to use Option

    // We can use a Box here because it has a pointer to data and
    // a fixed size

}
