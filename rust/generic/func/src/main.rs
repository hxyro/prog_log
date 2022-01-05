fn largest<A: PartialOrd>(list: &[A]) -> &A
{
    let mut largest = &list[0];
    for i in list{
        if i > largest{
            largest = i;
        }
    }
    largest
}

fn main()
{
    let num_list = vec![1,2,3,4,5,6,7,8,9,0];
    let char_list = vec!['a', 's', 'd', 'f','g', 'h', 'j', 'k'];

    println!("
                The largest number is {}\n
                The largest char is {}", largest(&num_list),largest(&char_list)
             );
}
