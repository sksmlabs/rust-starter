fn main() {
    println!("Hello World!");

    another_function(5);

    print_labelled_measurement(5, 'h');

    // statements do not return value, so you can't assign let to another let
    // The let y = 6 statement does not return a value, so there isn’t anything for x to bind to
    // let x = let y = 6;

    // Note that the x + 1 line doesn’t have a semicolon at the end
    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    let y = {
        let x = 3;
        x + 1
    };
    // try print y with semicolon after x+1;
    println!("The value of y is: {y}");

    let z = return_value();
    println!("The value of z is: {z}");

    let z = return_plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("Another Function.");
    println!("The value of x is: {}", x);
}

fn print_labelled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn return_value() -> i32 {
    // Note: No semicolon
    5
}

fn return_plus_one(x: i32) -> i32 {
    //Compile error if tried with semicolon
    x + 1
}