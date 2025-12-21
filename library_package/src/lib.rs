pub mod mod2;

pub mod mod3 {
    pub mod mod4;
    pub fn test3() {
        println!("3");
    }
}
pub fn test1() {
    println!("1");
}
