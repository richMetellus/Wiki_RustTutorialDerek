use rand::Rng;
use std::cmp::Ordering;


fn main() {
    // generate a random int between 1 and 100 (inclusive)
    let age = rand::rng().random_range(1..101);
    println!("Random age: {}", age);
    println!("-------------------------------------------------------------------");

    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("{} is Important Birthday", age);
    } else if age >= 65 {
        println!("{} is Important Birthday", age);
    } else {
        println!("{} is Not an Important Birthday", age);
    }
    println!("-------------------------------------------------------------------");
    
    // ternary operator 
    let mut my_age = age;

    let can_vote = if my_age >=18 {
        true // no semi-colon here as the statement has not ended
    } else {
        false // no semi-colon
    }; // semi-colon here to mark end of statement
    
    let can_vote_str = if can_vote {String::from("can VOTE!")} else {String::from("cannot VOTE!")};

    println!("You are {} years young and {}", my_age, can_vote_str);

    println!("-------------------------------------------------------------------");

    // match as conditional 
    match age {
        1..=18 => println!("{} match case 1: Important Birthday", age), // 1 through 18
        21 | 50 => println!("{} match case 2: Important Birthday", age), // 21 or 50
        65..=i32::MAX => println!("{} match case 3: Important Birthday", age), // > 65
        _ => println!("Not an Important Birthday"), // Default
    };

    println!("-------------------------------------------------------------------\n\n");


    // Compares age to valid age and cmp returns an Ordering which
    // has either the value Less, Greater, or Equal
    my_age = 18;
    println!("You are {} years young", my_age);
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You just gained the right to vote!"),
    };
    println!("-------------------------------------------------------------------");

}
