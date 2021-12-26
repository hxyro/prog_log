#[derive(Debug)]
enum IPADDRKIND {
    V4(string),
    V6(String),
}

fn main() {
    let home = IpAddr{
        kind: IPADDRKIND::v4
        address: String::from("0.0.0.0"),
    };
    let loopbaack = IpAddr{
        kind: IPADDRKIND::v6
        address: String::from("::1"),
    };
    let four = IPADDRKIND::v4;
    let six = IPADDRKIND::v6;
    println!("Hello, world!");
    route(IPADDRKIND::v4);
    route(IPADDRKIND::v6);
}
fn route(ip: IPADDRKIND) {}
