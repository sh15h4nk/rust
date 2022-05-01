// working with arrays in rust

fn main(){

    // basic initialization
    let mut a = [1, 2, 3];
    println!("a = {:?}", a);

    // initialization with size, type and data
    let mut b: [i32;3] = [4, 5, 6];
    println!("b = {:?}", b);

    // initilization with size, type and same data
    let mut c: [i32; 3] = [10;3];
    println!("c = {:?}", c);

    // from the above size and type can be omitted
    let mut d = [20;3];
    println!("d = {:?}", d);

    // setting values
    a[0] = 0;
    b[0] = 0;
    c[0] = 0;
    d[0] = 0;
    println!("After setting\na = {:?}\nb = {:?}\nc = {:?}\nd = {:?}", a, b, c, d);

    // getting values
    println!("elements at index 1 are a[1]= {}; b[1]= {}; c[1]= {}; d[1]= {};", a[1], b[1], c[1], d[1]);

    // lengths of arrays 
    println!("lengths:\n a = {}, b = {}, c = {}, d = {}", a.len(), b.len(), c.len(), d.len());

    // multidimentional arrays
    let e: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("e = {:?}", e);
    println!("e[0][1] = {}", e[0][1]);

}