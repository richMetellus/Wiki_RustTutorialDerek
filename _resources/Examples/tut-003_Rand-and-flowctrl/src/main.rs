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

}
