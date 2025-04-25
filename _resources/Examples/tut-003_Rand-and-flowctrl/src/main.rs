use rand::Rng;

fn main() {
    // generate a random int between 1 and 100 (inclusive)
    let random_num = rand::rng().random_range(1..101);
    println!("Random num: {}", random_num);
}
