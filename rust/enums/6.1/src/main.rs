#![allow(dead_code, non_camel_case_types, unused_variables)]
//  #[derive(Debug)]
//  enum IPADDRKIND {
//      V4(string),
//      V6(String),
//  }
//  
//  fn main() {
//      let home = IpAddr{
//          kind: IPADDRKIND::v4
//          address: String::from("0.0.0.0"),
//      };
//      let loopbaack = IpAddr{
//          kind: IPADDRKIND::v6
//          address: String::from("::1"),
//          };
//      let four = IPADDRKIND::v4;
//      let six = IPADDRKIND::v6;
//      println!("Hello, world!");
//      route(IPADDRKIND::v4);
//      route(IPADDRKIND::v6);
//  }
//  fn route(ip: IPADDRKIND) {}
//---------------------------------------------------

//  #[derive(Debug)]
//  enum IpAddr {
//      V_4(u8,u8,u8,u8),
//      V4(String),
//      V6(String),
//  }
//  fn main() {
//  
//      let home = IpAddr::V4(String::from("127.0.0.1"));
//      let home_4 = IpAddr::V_4(127,0,0,1);
//  
//      let loopback = IpAddr::V6(String::from("::1"));
//      println!("v4: {:#?}", home);
//      println!("v_4: {:#?}", home_4);
//      println!("v6: {:#?}", loopback);
//  }


#[derive(Debug)]
enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),

}

impl Message {
    fn call(&self){

    }
}

fn main()
{
    let m = Message::Write(String::from("ehhh"));
    m.call();
    println!("{:?}", m);
    let null: Option<u32> = None;
    println!("{:?}", null);

}
