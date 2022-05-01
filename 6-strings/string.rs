// working with strings in rust

fn main(){

    // from string type
    let a = String::from("This is just a string");
    println!("a = {}", a);

    // from to_string function
    let mut b = "This is another string".to_string();
    println!("b = {}", b);

    // lengths
    println!("Lengths: a = {}, b = {}", a.len(), b.len());

    // replace
    println!("{}", a.replace("just a", "third"));

    // appending to string
    b.push_str(" just appended");
    println!("{}",b);

    // spliting a string
    for tok in  "one,two,three".split(","){
        println!("{}", tok);
    }

    // iterating through each char
    for c in "EACH".chars(){
        println!("{}", c);
    }

    // to upper and lower case
    println!("{}", "upper".to_uppercase());
    println!("{}", "LOWER".to_lowercase());

    // char at index
    println!("{}", "qwerty".chars().nth(3).unwrap());

    // index of a char
    println!("{:?}", "qwerty".chars().position(|c| c == 't').unwrap());

    // starts with and ends with
    println!("{}", "This is a line".starts_with("This"));
    println!("{}", "This is a line".ends_with("This"));

}