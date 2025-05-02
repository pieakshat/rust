fn swap(a: &mut i32, b: &mut i32) {
    let temp: i32 = *a;
    *a = *b;
    *b = temp;
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let size: usize = arr.len();

    for i in 0..size {
        for j in 0..size - i - 1 {
            if arr[j] > arr[j + 1] {
                // swap(&mut arr[j], &mut arr[j + 1]); this won't work because rust does not allow two mutable references to the same object.
                let (left, right) = arr.split_at_mut(j + 1);
                swap(&mut left[j], &mut right[0]);
            }
        }
    }
}

fn main() {
    // take input
    let mut input = String::new();
    println!("Enter the number of elements in the array: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n: i32 = input.trim().parse().expect("invalid input");

    let mut arr: Vec<i32> = Vec::new();
    println!("Enter the elements of the array: ");
    for i in 0..n {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        let element: i32 = input.trim().parse().expect("invalid input");
        arr.push(element);
    }

    bubble_sort(&mut arr);
    println!("sorted array: {:?}", arr);
}
