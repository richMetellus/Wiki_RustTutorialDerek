use std::collections::HashMap;

fn main() {
    // ----- HASH MAPS -----

    // Create an empty hash map
    let mut heroes = HashMap::new();

    // Insert in hashmap (To overwrite use the same key)
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    // Iterate over hashmap
    for(k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }

    // Length of hashmap
    println!("Length : {}", heroes.len());

    // Check for key in hashmap
    if heroes.contains_key(&"Batman"){
        // Get value with key
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(_x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}
