fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

fn print_int(i: &i32) {
    println!("i32 is  {}", i);
}

fn main() {
    let mut arr = vec![2, 3, 5, 4, 1];
    // let mut arr = [2, 3, 5, 4, 1];
    selection_sort(&mut arr); //编译器会自动deref
    println!("{:?}", arr);
    let i = 5;
    print_int(&&&&i); //编译器会自动deref
}
