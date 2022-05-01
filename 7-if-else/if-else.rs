// working with if-else in rust

fn main(){
    
    // if and else
    if 7%2 == 0 {
        println!("7 is even");
    } else {
        println!("7 is odd");
    }

    // if without else
    if 8%4 == 0{
        println!("4 divides 8");
    }

    // multiple (if-elseif-else)
    let n = 0;
    if n < 0{
        println!("n is negative");
    } else if n > 0{
        println!("n is positive");
    } else{
        println!("n is zero");
    }
}