fn main() {
    let tup: (u16, f32, char) = (500, 6.4, 'c');
    println!("tuple: {:?}", tup); 
    
    let (x, y, z) = tup;
    println!("x: {} y: {} z: {}", x, y, z);

    let five_hundred = tup.0;
    println!("five hundred: {}", five_hundred);
}