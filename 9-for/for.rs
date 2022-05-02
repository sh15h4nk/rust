// working with for loops in rust

fn main(){

    // for in the given number range
    for i in 1..10 {
        println!("{}", i);
    }
    println!("\n");

    // for in the given number range where the end will be included
    for i in 1..=10 {
        println!("{}", i);
    }
    println!("\n");

    // iterating over a list
    let a = [1, 3, 2, 4];
    for i in a.iter(){
        println!("{}", i);
    }
    println!("\n");

    // interating over a mutable object(list)
    let mut b = [1, 2, 3, 4, 5];
    for i in b.iter_mut(){
        println!("{}", i);
    }
    println!("\n");


}