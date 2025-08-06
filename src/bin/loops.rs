fn main() {
    loop_with_returned_value();

    loops_nested_with_labels();
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