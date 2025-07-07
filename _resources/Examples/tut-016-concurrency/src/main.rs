use std::thread;
use std::time::Duration;

fn main() {
    // ----- CONCURRENCY -----
    // Concurrent programming involves executing different blocks of code
    // independently, while parallel programming is when different
    // code executes at the same instant of time. A thread handles scheduling
    // and execution of these blocks of code.

    // Common problems with parallel programming involve :
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution

    // Create a thread with spawn which takes a closure.
    // This thread will show how a thread does not necessarily to everything in
    // time.
    
    /* Commented out for next exercise.
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            // this tread cycle through and share outputting information to the screen
            println!("Spawned thread : {}", i);
            // Forces thread to sleep and allow another thread like main thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    // There are no guarantees on when the threads will execute and
    // that they will complete execution
    for count in 1..20 {
        println!("Main thread : {}", count);
        thread::sleep(Duration::from_millis(1));
    }

    // We call join here so that the main thread executes with thread1
    // unwrap handles the option Result which is Ok or Err
    thread1.join().unwrap();
    */
    
    // ----- BANK ACCOUNT EXAMPLE -----
    // We will create a bank account that multiple customers will try
    // to withdraw money from
    
    // Bank struct just contains current balance
    pub struct Bank {
        balance: f32
    }

    // Allows for withdrawing money
    // Pass a mutable reference so bank can be used elsewhere
    fn withdraw(the_bank: &mut Bank, amt: f32) {
            the_bank.balance -= amt;
        }


    // Create bank struct with $100 to start with
    let mut bank = Bank{balance: 100.00};
    withdraw(&mut bank, 5.00);
    println!("Balance : {}", bank.balance);

    // Create a customer thread that withdraws money
    // THIS WON'T WORK
    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 5.00)
    }

    // Can't do this closure may outlive the current function,
    // but it borrows `bank`, which is owned by the current function
    // If a thread can outlive the main function and the main function
    // has the bank which causes problems
    thread::spawn(|| {
        customer(&mut bank)
    }).join().unwrap();


}
