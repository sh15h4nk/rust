// working with command line arguments

use std::env;

fn main(){

    // collecting the args and turning them into vector of strings 
    let args: Vec<String> = env::args().collect();
    println!("CLI args are: {:?}", &args[1..]);

    //interating over args
    for ar in args[1..].iter() {
        println!("{}", ar);
    }

}