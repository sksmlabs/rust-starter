fn main() {
    loop_with_returned_value();

    loops_nested_with_labels();

    while_loop();

    while_loop_arr();

    for_loop();

    for_loop_with_range();
}

fn loop_with_returned_value() {
    println!("===== Loop with returned value =====");
    // returning value from the loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("===== END =====\n");
}

fn loops_nested_with_labels() {
    println!("===== Loop nested with labels =====");
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("===== END =====\n");
}

fn while_loop() {
    println!("===== While Loop =====");
    let mut number = 3;

    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }
    println!("===== END =====\n");
}

fn while_loop_arr() {
    println!("===== While Loop Array =====");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Could panic if the arr size change - thus prefer to use for loop
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    println!("===== END =====\n");
}

fn for_loop() {
    println!("===== For Loop Array =====");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
    println!("===== END =====\n");
}

fn for_loop_with_range() {
    println!("===== For Loop Range =====");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
    println!("===== END =====\n");
}