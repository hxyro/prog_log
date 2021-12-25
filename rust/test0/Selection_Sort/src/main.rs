#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

fn main()
{
    const SIZE: usize = 11;
    let mut arr: [u32; SIZE] = [1, 3, 56, 2, 45, 89, 34, 17, 82, 36, 90];
    println!("\n\nUnSorted array:");
    printarray(arr);

    let mut step = 0;
    let mut tmp = 0;
    'selectionSort: loop {
        let mut min = step;
        
        if step >= (SIZE - 1){
            break 'selectionSort;
        }

        let mut i = step + 1;
        loop {
            
            if i >= SIZE{
                break;
            }
            if arr[i] < arr[min]{
                min = i;
            }
            i = i+1;
        }
        
        tmp = arr[step];
        arr[step] = arr[min];
        arr[min] = tmp;
        step = step + 1;
    }

    println!("\n\nSorted array:");
    printarray(arr);
}

fn printarray(arr: [u32; 11])
{
    for i in (0..11) {
        print!("{} ", arr[i]);
    }
    println!("\n");
}
