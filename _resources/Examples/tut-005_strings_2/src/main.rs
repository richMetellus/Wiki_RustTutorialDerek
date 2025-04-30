fn main() {
    let greeting: &'static str = "Hello there";  // string literal
    println!("{greeting}");
    println!("address of greeting {:p}", &greeting);
    // greeting = "Hello there, earthlings"; // ILLEGAL since it's immutable 

    // is it still a string literal when it is mutable?
    let mut s: &'static str  = "hello"; // type is `&'static str`
    println!("s = {s}");
    println!("address of s {:p}", &s);
    // does the compiler coerce the type be &str or String?
    s = "Salut le monde!"; // is this heap-allocated or not? there is no `let` so not shadowing
    println!("s after updating its value: {s}"); // Compiler will not complain
    println!("address of s {:p}", &s);
    // Why does the code above work? since a string literal is a reference. 
    // A string literal is a string slice that is statically allocated, meaning 
    // that it’s saved inside our compiled program, and exists for the entire 
   // duration it runs. (MIT Rust book)

   let mut s1: &str = "mutable string slice";
   println!("string slice s1 ={s1}");
   s1 = "s1 value is updated here";
   println!("string slice after update s1 ={s1}");
}
