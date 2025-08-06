use std::io;

fn main() {
    learn_tuple();
    learn_array();
    
}

fn learn_tuple() {
    // Tutorial Link: https://www.youtube.com/watch?v=WbZdyDN_SE4
    println!("===== Tuple =====");
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
    println!("===== END =====\n");
}

fn learn_array() {
    // Tutorial Link: https://www.youtube.com/watch?v=1Nn5ivjXocE
    println!("===== Array =====");
    // Data type: Array

    // Fixed length, known at compile time
    let mut arr: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("arr: {:?}", arr);

    // Accessing array
    let arr_1 = arr[0];
    println!("arr_1: {}", arr_1);

    // Replacing array item
    arr[1] = 0;
    println!("arr: {:?}", arr);

    // Slicing (reference) of an array
    let arr_s = &arr[..3];
    println!("First 3 elements from array: {:?}", arr_s); 

    let arr_s  = &arr[3..7];
    println!("Middle 4 element from array: {:?}", arr_s); // index+1

    let arr_s = &arr[7..];
    println!("Last 3 elements from array: {:?}", arr_s);

    // Array with same items
    let arr: [&str; 4] = ["hello"; 4];
    println!("arr: {:?}", arr);

    println!("Please enter an array index.");

    let mut index = String::new();

    // Enter index from cli
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Access element form array
    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");

     // Invalid array element
     // if index is out of array length
    println!("===== END =====\n");
}