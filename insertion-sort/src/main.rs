use std::io::{self, Write};

pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the size of the array: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("read failed");
    let n: usize = input.trim().parse().expect("invalid number");

    let mut arr = Vec::with_capacity(n);
    println!("Enter the elements (one per line):");
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("read failed");
        let value: i32 = input.trim().parse().expect("invalid integer");
        arr.push(value);
    }
    println!("array: {:?}", arr);
    insertion_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
