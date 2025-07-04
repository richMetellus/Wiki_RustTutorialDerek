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

    // --You can pass closures to functions--
    // you can defined function inside another function just like python nested 
    // functions are allowed.

    // here we are passing 2 variables and a closure. You can define this function 
    // outside of main if you'd like. It is defined here simply for convenience reason.
    // here we label `func` as generic because we want to accept multiple different types
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        
        // push those 2 value into the function, and the different functions (add, or multiply)
        // can execute 
        func(a, b)
    }

    // create a closure which add a and b together
    let sum = |a, b| a + b;
    // create another closure which multiply these 2 value.
    let prod = |a, b| a * b;
    println!("----------Closure passed to function Exercise ----------");

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}
