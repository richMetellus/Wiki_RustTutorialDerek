// constant variables demo code:

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI : f32 = 3.141592;
    let age = "47"; // a string, implicitly typed. defined with double quotes
    let mut age: u32 = age.trim().parse() // LEGAL in rust to re-use the same variable name. Shadowing
        .expect("Age wasn't assigned a number"); 

    // now that the age is a u32, we can do operation.
    age = age + 1; // increment by 1.

    println!("I'm {} and I want ${} so I can buy some {}", age, ONE_MIL, PI);
}
