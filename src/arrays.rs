// arrays - fixed list with the same dtypes

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val 
    println!("Single value: {:?}", numbers[1]);

    // Get array length 
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated 
    println!("Array occupies {} bytes of memory", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}