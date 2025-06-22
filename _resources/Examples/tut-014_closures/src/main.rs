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
}
