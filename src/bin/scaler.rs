// Tutorial link: https://www.youtube.com/watch?v=hXUZt9Yj3EM

fn main() {
    // Signed Integers
    // range: -(2**(n-1)) ~ (2**(n-1))-1
    let i0: i8 = 1; // -128 ~ 127
    let i1: i16 = 1; // -32,768 ~ 32,767
    let i2: i32 = 1; // -2,147,483,648 ~ 2,147,483,647
    let i3: i64 = 1; // -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807
    let i4: i128 = 1; // 
    let i5: isize = 1; // 

    // Unsigned Integers
    // range: 0 ~ 2**n - 1
    let u0: u8 = 1; // 0 ~ 255

    // Floats
    let f0: f32 = 0.01;
    let f1: f64 = 0.01;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';
    let e: char = '😀';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // MIN AND MAX
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;
    println!("i32 min : {min_i}");
    println!("i32 max : {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;
    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 2;
    println!("overflow u: {u}");

    // checked_add - Panic or Some(x) '| None
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", u);

    // wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
}