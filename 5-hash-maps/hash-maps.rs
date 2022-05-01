// working with hashmaps (maps, dicts)
use std::collections::HashMap;

fn main(){

    let mut a: HashMap<String, String> = HashMap::new();
    println!("a = {:?}", a);

    // inserting elements
    a.insert("1".to_string(), "one".to_string());
    a.insert("2".to_string(), "two".to_string());
    a.insert("3".to_string(), "three".to_string());
    println!("a = {:?}", a);

    // getting values
    println!("a[\"1\"] = {}", a["1"]);

    // removing element
    a.remove(&"1".to_string());
    println!("After removing\na = {:?}", a);

    // length
    println!("length of a is {}", a.len());

    // checks for a key
    println!("key `1` exists in the map: {}", a.contains_key(&"1".to_string()));


}