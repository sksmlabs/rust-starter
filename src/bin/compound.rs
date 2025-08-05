// Tutorial Link: https://www.youtube.com/watch?v=WbZdyDN_SE4

use std::io;

fn main() {
    // Data type: Tuple
    let tup: (u16, f32, char) = (500, 6.4, 'c');
    println!("tuple: {:?}", tup); 
    
    // Destructure
    let (x, y, z) = tup;
    println!("x: {} y: {} z: {}", x, y, z);

    // Ignore with _
    let (_, b, _) = tup;
    println!("b: {}", b);

    // Empty tuple - unit type
    let t = ();

    // Nested tuple
    let nested = ((1.2, 'a'), (true, 1u32, 'b'), ());

    // Accessing tuple
    let five_hundred = tup.0;
    println!("five hundred: {}", five_hundred);

    println!("nested: {:?} {:?} {:?}", nested.0, nested.1, nested.2); 
    println!("nested: {}", nested.0.0);

    // Data type: Array
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);

    let arr_1 = arr[0];
    println!("arr_1: {}", arr_1);

    let arr = ["hello"; 5];
    println!("arr: {:?}", arr);

    // Invalid array element
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}