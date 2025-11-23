use std::io;

fn fib(n : u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut old_res = 0;
            let mut res = 1;
            for _i in 1..n {
                let temp = res;
                res = res + old_res;
                old_res = temp;
            }
            res
        }
    }
}

fn main() {
    println!("What number do you want ?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fialed to read line");
    let mut n:  Result<u32, _> = input.trim().parse();
    while n.is_err() {
        println!("Invalid input");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Fialed to read line");
        n = input.trim().parse();
    }
    let n : u32 = n.expect("should be good by now");
    let res = fib(n);
    println!("The nth fibonacci number is {res}");
}
