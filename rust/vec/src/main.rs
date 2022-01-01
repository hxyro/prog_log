#[allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3,4];
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(0);
    println!("{:#?}",v);
    println!("{}", &v[3]);
    println!("{}", &v[3]);
    match &v[4] {
        6 => println!("6"),
        3 => println!("3"),
        0 => println!("0"),
        _ => (),
    }
//    println!("{}", &v[10]);
    println!("{:?}", v.get(10));
    println!();
    for i in &v{
        println!("{}", i);
    }
    for i in &mut v{
        *i += 2;
    }
    println!();
    for i in &v{
        println!("{}", i);
    }
    #[derive(Debug)]
    enum Data {
        Int(i32),
        Float(f64),
        Txt(String)
    }
    let New = vec!{
        Data::Int(3),
        Data::Float(3.4),
        Data::Txt(String::from("dfsdfsf"))
    };
    println!("{:?}", New)
}
