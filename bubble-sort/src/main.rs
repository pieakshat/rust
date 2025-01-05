fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let size: usize = arr.len();
    for i in 0..size {
        for j in 0..size - i - 1 {
            if arr[j] > arr[j + 1] {
                // swap(&mut arr[j], &mut arr[j + 1]); this does not work because rust do not allow two mutable references to the same object
                // so we use the spli_at_mu method t osplit the array into two different mutable references and then swap the elemens.
                let (left, right) = arr.split_at_mut(j + 1); // two new mutable arrays are created left and right
                swap(&mut left[j], &mut right[0]);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the number of elements in the array: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Invalid input");

    let mut arr = Vec::new();
    println!("Enter the elements of the array: ");
    for i in 0..n {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let element: i32 = input.trim().parse().expect("Invalid input");
        arr.push(element);
    }

    bubble_sort(&mut arr);
    println!("sorered array: {:?}", arr);
}
