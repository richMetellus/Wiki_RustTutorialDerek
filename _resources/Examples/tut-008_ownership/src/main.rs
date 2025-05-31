#![allow(unused)]

fn print_str(x: String){
    println!("String passed: {}", x);
}

fn print_return_str(x: String) -> String{
    println!("String To return: {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is Happy");
    println!("Message : {}", name);
}

fn main() {
    // create 2 strings. If you want 2 copies use clone
    let str1: String = String::from("World");
    let str2: String = str1.clone();
    
    println!("Hello {}", str2);
    // can do this if  we had clone str1 into str2.
    println!("Hello {}", str1);
    
    // can pass str1 and str2 to function, no problem. they are still in scope 
    print_str(str1);
    print_str(str2);

}
