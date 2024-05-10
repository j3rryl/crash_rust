use std::cmp::Ordering;

fn main() {
    let number: i32 = 32;

    if number < 5 {
        println!("less than");
    } else {
        println!("greater than");
    }

    let condition = false;
    let num = if condition { 8 } else { 10 };

    println!("nums is {num}");

    match number.cmp(&33) {
        Ordering::Less => println!("less than"),
        Ordering::Greater => println!("greater than"),
        Ordering::Equal => println!("equal"),
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Loops");
        if counter == 3 {
            break counter * 2;
        }
    };

    println!("counter is {result}");

    me_loop();
    me_array();
    me_rev();
    abs_loop();
}

fn me_loop() {
    let mut number: u32 = 5;
    while number != 0 {
        println!("me number is {number}");
        number -= 1;
    }
}

fn me_array() {
    let a = [12, 13, 14, 15];
    for element in a {
        println!("value of a is {element}");
    }
}

fn me_rev() {
    for number in (1..4).rev() {
        println!("reverse number is {number}");
    }
}

fn abs_loop() {
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
}
