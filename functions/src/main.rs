// -----------------------------------------------------------------------------
// Functions
// -----------------------------------------------------------------------------

fn main() {
    let number = some_number();
    println!("{}", number);

    // Kind of acts as a function.
    let x = {
        let y = 3;
        y + 1
    };
    println!("{x}");

    // Return a closure.
    let closure = return_a_closure();
    println!("{}", closure(5.0));

    // Return a function.
    println!("{}", return_a_function()(1.0, 2.0));
}

// A basic function.
fn some_number() -> i32 {
    5 // no semicolon, means returns this value
}

// A function with an explicit return.
fn some_other_value() -> f64 {
    return 3.5;
}

fn return_a_closure() -> fn(f32) -> f32 {
    |x: f32| x + 1.0
}

fn return_a_function() -> fn(f32, f32) -> f32 {
    add
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}
