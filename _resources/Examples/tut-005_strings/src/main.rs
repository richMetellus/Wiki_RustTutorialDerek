fn main() {
    println!("-------------------------------------------------------------------");
    // Create an empty growable string
    let mut st1 = String::new();

    // Insert a character at the end of a string
    st1.push('A');

    // Insert a string at the end
    st1.push_str(" word"); // notice the space

    // Iterate through words by splitting at whitespace
    println!("Splitting st1 = \"{st1}\" - using whitespace delimiter");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    // Replace "A" from st1 to Another and bind the value to st2.
    println!("String substitution:  Replacing 'A' with \"Another\" in st1 = \"{st1}\" ");
    let st2 = st1.replace("A", "Another");
    println!("st2 = \"{}\"", st2);
    println!("is st1 still in scope? Yes; st1 ={st1}");
}
