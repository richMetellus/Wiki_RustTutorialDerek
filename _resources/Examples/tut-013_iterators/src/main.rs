fn main() {
    let arr_it = [1, 2, 3, 4];

    // cycle through the array.
    // An iterator cycles through values by borrowing, so the collection
    // is not moved (You can't change values). Ownership is not moved.
    print!("Iterate through all the value in the array \n[");
    for val in arr_it.iter(){
        print!("{},", val);
    }
    println!("]");

    // you can also do ``arr_it.into_iter()``, in this situation, you'll consume
    // the collection but you'll no longer able to use the collection.

    println!("You can also create an iterator");
    let mut iter1 = arr_it.iter();
    println!("And call for each value with next");
    println!("1st : {:?}", iter1.next());
}
