// -----------------------------------------------------------------------------
// Ownership
//
// Rules:
//
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
//
// -----------------------------------------------------------------------------

fn main() {
    // Stored in stack memory.
    let _a = 1;
    let _str = "hello";
    {
        let _temp = 22;
    }
    // _temp got "popped" off the stack.

    // Stored in heap memory, with pointers on stack.
    let b = Box::new(2);
    let mut string = String::from("Hello");
    string.push_str(" World!");
    println!("{}", string);

    // Changing ownership of a heap value.
    let c = b;
    println!("{}", c);
    //println!("{}", b); // compile error! c owns the heap value now! b is
    // no longer valid.

    // Copying a stack value.
    let m = 123;
    let _n = m;

    // Creating a new variable with the same value from c. This copies the
    // stack and/or heap. It's a completely independent value from c.
    let d = c.clone();
    println!("{}", d);

    let mut some_string = String::from("hello");
    // Works! But not ideal.
    takes_ownership(some_string.clone());

    takes_refernce(&some_string);

    modifies_string(&mut some_string);

    takes_ownership(some_string);
    //println!("{}", some_string); // compile error! moved ownership to takes_ownership

    // Can only have 1 mutable reference!
    let mut message = String::from("hello");
    let _r1 = &mut message;
    //let _r2 = &mut some_string; // compile error!
    let _r3 = &message; // ok!
    let _r4 = &message; // ok!

    // Dangling references:
    //let danger = dangle(); // compile error! Dangling reference!
}

// -----------------------------------------------------------------------------

fn takes_ownership(src: String) {
    println!("takes_ownership: {}", src);
}

fn takes_refernce(src: &String) {
    println!("takes_reference: {}", *src);
}

fn modifies_string(src: &mut String) {
    src.push_str(" world!");
}

// compile error!
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}
