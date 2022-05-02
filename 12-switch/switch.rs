// working with switch in rust

fn main(){

    // literal matching
    let a = 5;
    match a {
        1 => println!("Its one"),
        2 => println!("Its two"),
        3 => println!("Its three"),
        4 => println!("Its four"),
        5 => println!("Its five"),
        _ => println!("match not found"),
    }
    
    let b = "1";
    match b {
        "0" => println!("String 0"),
        "1" => println!("String 1"),
        _ => println!("Invalid string"),
    }
    
    // conditional matching 
    let c = 19;
    match c{
        0 | 1 | 2 | 3 | 4  => println!("Its less than or equal to 5"),
        5 | 6 | 7 | 8 | 9 => println!("Its between 5 to 9"),
        _ => println!("Nothing matched"),
    }

    // matching with range
    let d = 10;
    match d{
        0..=10 => println!("Between 0 - 10"),
        11..=20 => println!("Between 11 - 20"),
        _ => println!("No case matched"),
    }

    // matching bools
    let e = false;
    match e{
        true => println!("Its true"),
        false => println!("Its false"),
    }
}