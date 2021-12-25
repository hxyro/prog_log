#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn main() {
    //normal
//    let width1 = 30;
//    let height1 = 60;
    //tupel
//    let ar: (u32,u32) = (30 ,60);
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!(
        "the area of rectangle is {} square pixels.",
//        area(width1, height1)
//        area(ar)
        area(&rect1)
    );
    dbg!(&rect1);
}

//fn area(w: u32, h: u32) -> u32
//fn area(ar: (u32,u32)) -> u32
fn area(rect: &Rectangle) -> u32
{
//    w*h
//    ar.0*ar.1
    rect.width*rect.height
}
