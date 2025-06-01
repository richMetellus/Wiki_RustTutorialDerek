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
    println!("name is still valid {}", name);
    name.push_str(" Happy");
    println!("Final Mutated Message : {}", name);
}

fn change_string_2(mut name: String){
    name.push_str(" is Happy");
    println!("Mutated Message : {}", name);
}

fn main() {

    // --- Ownership Rules ---
    // 1. Each value in Rust has a variable that's called its owner.
    //    * This ensure single ownership
    // 2. There can only be one owner at a time 
    //    * This is to move ownership (value) to another scope for better efficiency
    // 3. When the owner goes out of scope, the value will be dropped.
    //    * by default, rustc will call a tiny function `drop` to free heap allocated data
    //    *  this for scope. scope is code enclosed with ``{}``
    
    { // string slice slc is not valid here, it is not yet declared.
        let slc = "hello"; // slc is valid from this point forward.
        //  "hello" is NOT data allocated on the heap. It is saved with 
        // the binary, most likely in .rodata section. slc is  pointer to static memory

        // -- do stuff with slc. --

    }// this scope is now over, and slc is no longer valid. 

    let x = vec![1, 2, 3]; // a vector is a heap allocated object/data structure. 
            // You can think of x as a pointer variable on that stack that 
            // points to the growable data on the heap. 
            // Data on heap = what x own, i.e vec![1, 2, 3].
            // ownership of x =  i.e value of x = i.e memory address where the heap data start.
            // so x holds a memory address. x is a pointer. x is saved on the stack.
    
    println!("address of x: {:p}", &x); // this will print what memory location x represents i.e 
                                        // will print the program memory address referred to as x in code 
                                        // This will NOT print the heap memory address x points to.
    
    // let y = x; // ownership of x is moved to y. i.e the value of x is moved to y.
                  // i.e if x = 0xdeadbeef, where 0xdeadbeef is the start address of 
                  // the heap data vec![1,2,3], then y would have also hold as value 
                  // 0xdeadbeef. i.e y would point to same heap data that x already pointing
                  // can't do `let y = x;` since this would have violated the single ownership rule.
                  // at one point of time x might hold for address 0xdeadbeef, then  later on 
                  // it might hold heap data memory address 0xbeefdead
                  // data structure placed on the heap can be moved around by the allocator 
                  // specially if the data structure object grows or shrinks and the original 
                  // size that was allocated for the object/data structure is too small or 
                  // insufficient. 
    
    let y = &x; // y is a reference to x. LEGAL. think of y as a c pointer to pointer
                           // y borrow what x owns?? x does not lose ownership of the heap data.

    println!("address of y: {:p}", &y); // address of y is different than address of x
    println!("x refer to: {:p}", x.as_ptr());
    println!("y refer to: {:p}", y.as_ptr());
    println!("--the next 2 lines will print the same address--");
    println!("x as raw pointer using p: {:p}", x.as_ptr());
    println!("y as raw pointer using p: {:p}", y.as_ptr()); 
    
    println!("--the next 2 lines will print the same address--");
    println!("x as raw pointer using ?: {:?}", x.as_ptr());
    println!("y as raw pointer using ?: {:?}", y.as_ptr()); 
    
    println!("--the next 2 lines will NOT print the same address--");
    println!("&x as raw pointer using p: {:p}", &x.as_ptr());
    println!("&y as raw pointer using p: {:p}", &y.as_ptr());
    
    println!("--the next 2 lines will print the same address--");
    println!("&x as raw pointer using ?: {:?}", &x.as_ptr());
    println!("&y as raw pointer using ?: {:?}", &y.as_ptr());

    println!("--------------- Strings and Ownership -------------------------");
    // create 2 strings. If you want 2 copies use clone
    let str1: String = String::from("World"); // "World" is allocated on the heap. str1 points to 
                                              // first character in "World". str1 is // Heap pointer
    // cannot do this. thus it is commented
    // let str2: String = str1;
    // but can do this
    let str2: String = str1.clone();
    println!("Hello {}", str2);
    // can do this if  we had cloned str1 into str2.
    println!("Hello {}", str1);
    // str2 still owned the value "World" (copy of str1)
    println!("printing str2 again no problem {}", str2);

    // can pass str1 and str2 to function, no problem. There are still in scope 
    // before the function call. Calling the function means ownerships are moved to 
    // another scope.
    print_str(str1);
    print_str(str2); // value (heap data) is moved into the print_str function. str2 no longer the owner. 
                    // str2 no longer owned the heap data "World". Head data is 
                    // Out of scope so Rust compiler frees it.

    // str1 and str2 were put into another scope, so can't use them anymore.
    // whether we pass str1 or str2 code won't compile
    // this is to remove multiple references to the data on the heap
    // let str3 = print_return_str(str1); // uncomment this line to see error

    // str2 is no longer the owner of the value "World" since it was moved into another scope.
    // println!("printing str2 will cause error {}", str2); // uncomment this line to see error

    // if a function is going to borrow a reference it cannot change it unless we 
    // create a mutable version of it. You can only create one mutable version 
    // inside of a function
    //  ^ mut keyword
    let mut str4 = String::from("Ricky");
    change_string(&mut str4); // looking at this function &mut we know str4 can be changed by the function
                                    // this function will only work on mutable reference

    // str4 is still valid 
    println!("str4 is still valid but mutated: {str4}");

    let mut str5 = String::from("Dopa");
    change_string_2(str5); // LEGAL

    // println!("str5 is no longer valid: {str5}"); // uncomment to see error


}
