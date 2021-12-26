//#[derive(Debug)]
//enum UsState {
//    Alabama,
//    Alaska,
//}
//#[derive(Debug)]
//enum Coin {
//    Penny,
//    Nickel,
//    Dime,
//    Quarter(UsState),
//}
//
//fn Value_Count(coin: Coin) -> u32
//{
//    match coin {
//        Coin::Penny => 1,
//        Coin::Nickel => 5,
//        Coin::Dime => 10,
//        Coin::Quarter(state) => {
//            println!("{:?}", state);
//            25
//        },
//    }
//}
//fn main() {
//    let cc = Coin::Penny;
//    println!("{:?}",Value_Count(cc));
//    println!("{:?}",Value_Count(Coin::Quarter(UsState::Alabama)));
//}
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}
