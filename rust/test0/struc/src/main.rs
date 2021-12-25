#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: &str, username: &str) -> User
{
    let mut email = email.to_string();
    let mut username = username.to_string();
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User{
        email: String::from("huehueheu@gmail.com"),
        username: String::from("huehuehue"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("dfdfdfsdf@gmail.com");

//  let mut em: String = String::from("ehhhhh@gmail.com");
//  let mut em1: String = String::from("ehhh");
    let mut user2 = build_user("dfgdfgdfg" , "sdfsdf");

    let user3 = User{
        email: String::from("user3@gmail.com"),
        username: String::from("dsdf"),
        ..user2
    };

    println!("{:#?}", user1);
    println!("{:#?}", user3);
    println!("{:#?}", user2);
    #[derive(Debug)]
    struct color(i32,i32,i32);
    #[derive(Debug)]
    struct point(i32,i32,i32);

    let black = color(0,0,0);
    let x = point(1,0,0);
    println!("{:?}", black);
    println!("{:?}", x);
}
