// Tutorial link: https://www.youtube.com/watch?v=b4GbXzB7PYo

#![allow(unused)]

fn main() {
    // Immutable variables
    let x: i32 = -123;
    // Compile error: Cannot assign twice to immutable variable
    // x += 1;
    println!("x : {}", x);

    // Mutable variables
    let mut y: i32 = -123;
    y += 1;
    println!("y : {}", y);

    // Debug type of variable
    // let z: () = 123;
    // compile error: expected `()`, found integer

    // Constant
    const NUM: u32 = 1; 
    // Compile error: cannot assign to this expression
    // NUM += 1;

    // Redeclaring variable
    let w: i32 = -1;
    let w: bool = true;
    println!("w : {}", w);

    // Rust infer the type itself
    let v: Vec<_> = vec![1,2,3];
    println!{"v : {:?}", v};
}