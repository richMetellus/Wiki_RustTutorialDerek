use std::thread;
use std::time::Duration;

// A smart pointer Rc<RefCell<T>> allows multiple owners with mutable
// access to the same data
use std::cell::RefCell;
use std::rc::Rc;

// Arc<T> provides shared ownership of a value
// Mutex blocks threads waiting for lock to be available
use std::sync::{Arc, Mutex}; // atomic reference counter which provide share ownership of a value.

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
        balance: f32,
    }

    /* --------------- BAD CODE ----------
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

            ------------------- BAD CODE END -------------------
    */

    /*  GOOD CODE START */

    // withdraw takes a reference to a bank, which is a shared ownership.
    // this takes a mutex which going wait, block thread until the_bank is available.

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance : {} Withdrawal a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew {} Current Balance {}",
                amt, bank_ref.balance
            );
        }
    }


    // customer of the_bank comes to withdraw money. Can only withdraw $5.
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank_branch: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    // Creates 10 customer threads
    let handles = (0..10).map(|_| {
        // Clone duplicates an the bank_branch object
        let bank_ref = bank_branch.clone();
        thread::spawn(|| customer(bank_ref))
    });

    // Wait for all customers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", bank_branch.lock().unwrap().balance);

    // ----- INSTALLATION ------
    // Install rustup on Mac or Linux
    // curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
}
