use std::io;

fn main() {
    let vowels: [char;6] = ['a','e','i','o','u','y'];
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input =    input.trim();
    let mut res = String::new();
    for word in input.split_whitespace(){
        let mut ite = word.chars();  
        if let Some(first) = ite.next() {
            if vowels.contains(&first){
                let new_word = format!("{word}-hay ");
                res.push_str(&new_word);
            } else {
                let rest : String = ite.collect();
                let new_word = format!("{rest}-{first}ay ");
                res.push_str(&new_word);
            }
        }
    }
    println!("{res}");
}
