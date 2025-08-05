fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    condition_without_bool();
    condition_with_elseif();
    condition_to_variable();
}

fn condition_without_bool() {
    let number = 3;

    // compile error because if expression should always be bool
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn condition_with_elseif() {
    let number = 6;

    // executes only the first true body
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn condition_to_variable() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // compile error: if mismatch types in arms of if and else
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");
}