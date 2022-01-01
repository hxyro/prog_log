use std::io::{stdin, stdout, Write};

fn main() {
    print!("Enter text: ");
    let mut text = String::new();
    let mut text1 = String::new();

    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut text).expect("Failed to read line");
    
    let vowel = String::from("aeiou");

    for i in text.split_whitespace() {
        if vowel.contains(&i[0..1]){
            text1 = text1 + i + "-hay ";
        }
        else {
            text1 = text1 + &i[1..] + "-" + &i[0..1] + "ay ";
        }
        }
    println!("pig latin: {}", text1);
}
