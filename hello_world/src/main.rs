fn main() {
    let a: i32 = 1;
    let b: f64 = 2.2; // immutable by default
    const c: f32 = 1.2; // compile time constant
    let mut d: i32 = 2; // mutable

    let mut condition = true;

    while condition {
        static e: u16 = 2; // will only be created once, regardless of how many times the loop iterates
        condition = false;
    }

    d = 22;

    let d_ptr = &d;

    let strings: Vec<&str> = vec!["a", "b"];

    println!("{}", strings[1]);

    println!("d_ptr: {}", d_ptr); // prints 22
    println!("d_ptr is 22?: {}", *d_ptr == 22); // prints 22

    println!("{}", return_hello_world());
}

fn return_hello_world() -> &'static str {
    "hello world!!!"
}

type ReturnHelloWorld2Fn = fn() -> String;

fn return_hello_world_2() -> String {
    let result = String::from("hello world!!!");
    let parts: Vec<&str> = result.split(' ').collect();
    parts[0].to_string()
}

struct Object {
    name: String,
    position: (f64, f64, f64),
}

impl Object {
    fn move_position(&mut self, x: f64, y: f64) {
        self.position = (self.position.0 + x, self.position.1 + y, self.position.2);
    }
}
