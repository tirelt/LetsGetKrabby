#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 3;
    let rect1 = Rectangle {
        width: dbg!(scale * 10),
        height: 50,
    };
    let _rect2 = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
}

