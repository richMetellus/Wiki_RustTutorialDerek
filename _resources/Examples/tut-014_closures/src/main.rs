fn main() {
    // ----- CLOSURES -----
    // A closure is a function without a name and they are sometimes
    // stored in a variable (They can be used to pass a function into
    // another function)
    // let var_name = |parameters| -> return_type {BODY}

    // Create a closure that defines if someone can vote
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote : {}", can_vote(8));

    // Closures can access variables outside of its body with borrowing
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    // change the value
    samp1 = 10;
    println!("samp1 initial value= {}", samp1);

    // You can change values if you mark the closure mutable
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 after change inside closure = {}", samp1);
    samp1 = 10;
    println!("samp1 after setting it back to original value = {}", samp1);
}
