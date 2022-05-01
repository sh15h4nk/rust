// working with rust variables

fn main(){

    // strings declaration
    let a = String::from("This is a string");
    println!("a = {}",a);

    // declaring multiple variables for both mutable and immutable types
    let (b, c, d) = (12, 23, 31);   //immut (default)
    println!("b = {}\nc = {}\nd = {}", b, c, d);

    let (mut e, mut f, mut g) = (78, 89, 97);
    println!("e = {}\nf = {}\ng = {}", e, f, g);

    (e, f, g) = (1, 2,3);
    println!("After updating mutable variables\ne = {}\nf = {}\ng = {}", e, f, g);

    // bool variables
    let h = true;
    println!("h = {}", h);

    // constants
    const PIE: f32 = 22.0/7.0;
    println!("pie = {}", PIE);

    // shadowing 
    let j = 4;
    println!("Initial j = {}", j);
    let j = j + 1;
    println!("After shadowing j = {}", j);


}