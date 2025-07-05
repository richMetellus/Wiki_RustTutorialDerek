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
    // struct TreeNode<T> {
    //     pub left: TreeNode<T>,
    //     pub right: TreeNode<T>,
    //     pub key: T, // this caused error
    // }

    // We have other problems in that Binary Trees eventually end
    // and Rust doesn't like Null values so we have to use Option

    // We can use a Box here because it has a pointer to data and
    // a fixed size

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T
    }

    // Create functions for creating nodes and adding left & right
    impl<T> TreeNode<T> {
        // the key is the value assigned to the node.
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None, // bring new node, so typically they have none.
                right: None,
                key,
            }
        }
        // to assign a left tree node.
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        // to assign a right tree node.
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    // Used to test original, single node binary tree.
    let mut boss = TreeNode {
        left: None,
        right: None,
        key: 50,
    };

    // Create the root node with left and right

    /*
             1
            / \
           2    3
          / \  / \
          4 5  6  7
     */

    let node2 = TreeNode::new(2)
    .left(TreeNode::new(4))
    .right(TreeNode::new(5));
    
    let node3 = TreeNode::new(3)
    .left(TreeNode::new(6))
    .right(TreeNode::new(7));

    let binary_tree1 = TreeNode::new(1)
    .left(node2)
    .right(node3);

}
