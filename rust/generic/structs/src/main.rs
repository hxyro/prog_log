fn main() {
    #[derive(Debug)]
    struct Point<T,A,K> {
        x: T,
        y: A,
        z: K
    }
    let int = Point {x: 12, y: String::from("dfjlgjldfgf"), z: "ehh"};
    println!("{:?}", int.x);
    println!("{:?}", int.y);
    println!("{:?}", int.z);
}
