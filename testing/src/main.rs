fn test()->i32 {
    let a = 1;
    let _boloss :f32 = 1.1;
    a
}
fn main() {
    let res = test();
    println!("value is {res}");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
