// Working with data types in rust

fn main(){

    // concating strings
    println!("{}", concat!("This is a line from", " rust"));

    // Integers and floats
    println!("1 + 1 = {}", 1+1);
    println!("7.0 / 3.0 = {}", 7.0/3.0);

    // booleans
    println!("{}",true && false);    //false
    println!("{}",true || false);    //true
    println!("{}",!true);    //false

}