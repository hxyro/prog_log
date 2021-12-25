#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32
    {
        self.width*self.height
    }
    fn sum(&self) -> u32
    {
        self.width+self.height
    }
    fn can_hold(&self, a: &Rectangle) -> bool
    {
        if self.width > a.width && self.height > a.height
        {
            true
        }
        else {
            false
        }
    }
    fn square(size: u32) -> Rectangle
    {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 10,
        height: 60,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
    println!("sqr: {:#?}", Rectangle::square(3));
}
