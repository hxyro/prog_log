fn main() {
    use std::collections::HashMap;
    let mut hash = HashMap::new();
    hash.insert(String::from("ehhh"), 4);
    hash.insert(String::from("ehhhhh"), 44);
    println!("hash\n {:#?}", hash);

    let word = vec![String::from("ehhh"),String::from("ehhhhh")];
    let num = vec![4, 44];
    let mut hash: HashMap<_,_> = word.into_iter().zip(num.into_iter()).collect();
    println!("hash\n {:#?}", hash);
    println!("hash\n {:?}", hash.get("ehhh"));
    hash.entry(String::from("ehhh")).or_insert(5);
    hash.entry(String::from("uhhh")).or_insert(5);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
