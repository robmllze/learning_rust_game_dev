// -----------------------------------------------------------------------------
// Variables
// -----------------------------------------------------------------------------

use std::io::{self, Write};

fn main() {
    // Variables are saved in stack memory.

    // Mutability and immutability.
    let x = 5;
    let mut y = x;
    y += 1;
    let z = x + y;
    println!("x={}, y={}, z={}", x, y, z);
    let _a: u32 = 1;
    let _b: i32 = -1;
    let _c: f32 = 1.0;

    // Comonstants.
    const PI: f64 = std::f64::consts::PI;
    const E: f64 = std::f64::consts::E;

    println!("E={E} PI={PI}");

    // Characters.
    let c = 'c';
    let cc = c as u8;
    println!("c={c} cc={cc}");

    // String literal.
    let my_string: &str = "hello";
    println!("my_string={my_string}");

    // Shadowing variables.
    let spaces = "   ";
    println!("{}", type_of(&spaces));
    let spaces = spaces.len();
    println!("{}", type_of(&spaces));
    {
        let spaces = 's';
        println!("{}", type_of(&spaces));
    }
    println!("{}", type_of(&spaces));

    // Booleans.
    let t = true;
    if t {
        println!("t is true");
        if !t == false {
            println!("!t is false");
        }
    }

    // Tuples.
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = x;
    println!("a={a}, b={b}, c={c}");
    println!("x.1={}", x.1);

    // Arrays.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..a.len() {
        println!("a[{}]={}", i, a[i]);
    }
    let a = [3; 5];
    for i in a.iter() {
        println!("a={}", i);
    }

    // String to int.
    let mut input = String::new();
    print!("Enter some index value: ");
    io::stdout().flush().expect("Failed to flush stdout!"); // Force the buffer to flush
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!"); // Panic with this message on failure.
    let index: usize = input
        .trim()
        .parse()
        .expect("Index entered was not a number!");
    println!("You entered {A} and the value here is {B}", A = index, B = a[index]);
}

// -----------------------------------------------------------------------------

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
