use std::io;

fn main() {
    let vowels: [char;6] = ['a','e','i','o','u','y'];
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    input.pop();
    let init_string = input.clone();
    let mut ite =input.chars();  
    let first = ite.next().unwrap();
    let mut new_string = String::new();
    let start_with_vowel = vowels.contains(&first);
    if start_with_vowel{
        new_string.push(first);
    } 
    for c in ite{
        new_string.push(c);
    }
    if start_with_vowel{
        new_string.push_str("-hay");
    } else {
        new_string.push('-');
        new_string.push(first);
        new_string.push_str("ay");
    }
    println!("{init_string} -> {new_string}");
}
