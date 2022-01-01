fn main() {
    let hello = "Здравствуйте";

    let s = &hello[0..6];
    println!("{}",s);
    for c in "नमस्ते".chars() {
    println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
    println!("{}", b);
}
}
