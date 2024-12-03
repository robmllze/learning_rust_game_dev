// -----------------------------------------------------------------------------
// Iterators and Closures
// -----------------------------------------------------------------------------

use std::thread;

fn main() {
    println!("Hello, world!");

    // true ? 1 : 2 isn't a valid expression, but this is.
    let _a = true;
    let _b = if true { 1 } else { 2 };

    // Functions and closures.
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1;
    let a = add_one_v2(1);

    // Closures may borrow implicitly.
    let mut counter = 0;
    let mut add_one_v5 = || {
        counter += 1;
        counter
    };
    add_one_v5();
    println!("{}", counter);

    // Closures may move explicitly (stack).
    let mut counter_2 = 0;
    let mut add_one_v6 = move || {
        counter_2 += 1;
        counter_2
    };
    add_one_v6();
    println!("{}", add_one_v6());
    add_one_v6();
    println!("{}", counter_2);

    // Closures may move explicitly (heap).
    let mut name = "Bob".to_string();
    let mut do_sum_sum = move || {
        name.push_str(" Marley");
        name
    };
    //do_sum_sum();
    println!("{}", do_sum_sum());
    // do_sum_sum(); // error!
    // println!("{}", name); // error!

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Move list to a new thread.
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

// Closures and generics. FnOnce is a trait for functions that can be called
// only once. Fn is a trait for functions that can be called multiple times.
// FnMut is a trait for functions that don't move captured values.
impl<T> OptionClone<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
