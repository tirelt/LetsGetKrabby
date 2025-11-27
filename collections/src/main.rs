use std::io;
use std::collections::HashMap;
fn main() {
    println!("hello");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut v: Vec<i32> = words.iter().map(|x| x.parse().unwrap()).collect();
    v.sort();
    let median = v[v.len()/2];
    let mut count_map: HashMap<i32,i32> = HashMap::new();
    println!("The median is {median:?}");
    for el in v{
        let el_ref = count_map.entry(el).or_insert(0);
        *el_ref += 1;
    }
    println!("Count map\n:{count_map:?}");

}