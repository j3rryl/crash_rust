fn main() {
    another_function();
    let x = five();
    let y = plus_value(10);
    println!("x is {x}");
    println!("y is {y}");
}

fn another_function() {
    println!("Me func");
}

fn five() -> i32 {
    5
}

fn plus_value(value: i32) -> i32 {
    value * 5
}
