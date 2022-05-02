// workng with slicing

fn main(){

    // string slicing
    let s = String::from("Hello there");
    println!("{}", &s[0..5]);   //hello
    println!("{}", &s[6..11]); //world

    println!("{}", &s[..5]); //hello
    println!("{}", &s[6..]); //world

    println!("{}", &s[..]); //whole string

    // slicing array
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", &a[2..4]);

}