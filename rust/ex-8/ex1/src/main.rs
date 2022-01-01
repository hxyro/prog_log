#[allow(unused_variables, unused_mut, unused_assignments)]
use std::collections::HashMap;

fn main(){
    let mut arr: Vec<i32> = vec![23,4,45,67,776,87,90,45,67,34,32,56,7,67,645,23,2676,76,1,1,1,1,1];
    println!("mean == {}", mean(&arr));
    println!("median == {}", median(&mut arr));
    println!("mode == {}", mode(&arr));
}
fn mean(arr: &Vec<i32>) -> i32 {
    let mut mean = 0i32;
    let mut count = 0i32;
    for i in arr{
        mean += i;
        count += 1;
    }
    mean/count
}
fn median(arr: &mut Vec<i32>) -> i32 {
    let length = arr.len()/2;
    arr.sort();
    arr[length]
}
fn mode(arr: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in arr{
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut k = 0;
    let mut j = 0;
    for (key, value) in &map {
        if *value > k {
            k = *value;
            j = **key;
        }
    }
    j
}
