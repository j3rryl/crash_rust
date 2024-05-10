fn main() {
    const MY_TYPE: u32 = 23 * 18;
    println!("Your const number is {MY_TYPE}");
    let x = 23;

    {
        let x = x + 23;
        println!("x is {x}");
    }
    println!("x final is {x}");

    // Data types:
    // Integer types:
    // u8, i8, 16, 32,64,128
    // Floating tyoes
    // f32
    // Bool
    // bool
    // Character
    // char

    // Compound types -> arrays and tuples

    // let tup = (500, 6.4, 1);
    // let (a, b, c) = tup;

    let tup: (u32, f64, u8) = (500, 6.4, 1);
    let (a, _b, _c) = tup;

    // let me = tup.0 -> arraylike
    println!("Value of a is {a}");

    // Arrays
    // let a = [1,2,3,4,5];
    // let a = [3; 5] -> 3 five times
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let accessed = a[4];
    println!("Array is {accessed}")
}
