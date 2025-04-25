use rand::Rng;

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
    let my_age = age;

    let can_vote = if my_age >=18 {
        true // no semi-colon here as the statement has not ended
    } else {
        false // no semi-colon
    }; // semi-colon here to mark end of statement
    
    let can_vote_str = if can_vote {String::from("can VOTE!")} else {String::from("cannot VOTE!")};

    println!("You are {} years young and {}", my_age, can_vote_str);

    println!("-------------------------------------------------------------------");

}
