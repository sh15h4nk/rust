// working with closures

fn main(){
    
    // without type defination
    let closure_increment = |i| i+1;
    println!("{}", closure_increment(5));

    // with type defination
    let c = |i: i32| -> i32{ i+1};
    println!("{}", c(5));

}