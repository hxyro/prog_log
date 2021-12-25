#![allow(unused_variables)]
fn main() {
    let name: &'static str = "huehuehuehue";
    println!("{}", name);

    let mut name2: String = String::new();
    let mut name3: String = String::from("fgs hkg skjghkjghgksg");
    name2 = name2 + "eh ";
    own(&name2);
    name2.push_str("wtf");
    name3.push_str("wtf");
    println!("{}", name2);

    let name: &str = &name2[..4];
    println!("{}", name);
    for c in name3.chars() {
        print!("{} ", c);
    }
    println!("");
    let mut n = 3;
    sqr(&mut n);
    println!("{}", get_len(&name3));
    println!("{}", n);

}

fn get_len(s: &str) -> usize {
    s.len()
}
fn own(s: &String){
    println!("{}",s);
}

fn sqr(n: &mut u32)
{
    *n = *n * *n;
}
