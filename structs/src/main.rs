// -----------------------------------------------------------------------------
// Structs
// -----------------------------------------------------------------------------

fn main() {
    // Creating a struct.
    let mut user = UserPub {
        id: dbg!(String::from("u_123456")),
        r#ref: String::from("users/u_123456"),
        display_name: String::from("Robert Mollentze"),
        email: String::from("robmllze@gmail.com"),
    };

    // Updating a struct value.
    user.display_name = String::from("Robert Mollentze, Jr.");

    // Creating a struct using a function.
    let user = build_user(
        String::from("robmllze@gmail.com"),
        String::from("Robert Mollentze"),
    );

    // Creating a struct from an existing struct, with a different email value.
    let user2 = UserPub {
        email: String::from("robmllze+test@gmail.com"),
        ..user
    };

    // Tuple structs.
    struct ColorRGBA(u8, u8, u8, u8);
    struct PointXYZ(f32, f32, f32);

    let some_point = PointXYZ(3.0, 4.0, 5.0);
    let red = ColorRGBA(255, 0, 0, 255);

    let _position = Position {
        x: 1.0,
        y: dbg!(2.0 * 2.0),
        z: 3.0,
    };

    // Position implements the debug trait, so it can be printed.
    println!("{:?}, {:#?}", _position, _position);

    // Creating a few shapes!
    let rect = Rectangle::new(3.0, 4.0);
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    println!("Rectangle area: {}", rect.area());
    println!("Circle area: {}", circle.area());
    println!("Triangle area: {}", triangle.area());
}

// -----------------------------------------------------------------------------

fn build_user(email: String, display_name: String) -> UserPub {
    let id_placeholder = "u_123456";
    let ref_placeholder = format!("users/{}", id_placeholder);
    UserPub {
        id: id_placeholder.to_string(),
        r#ref: ref_placeholder.to_string(),
        // "displayName: displayName" isn't necessary, since the key name
        // is the same as the variable name.
        display_name,
        email,
    }
}

struct UserPub {
    id: String,
    // Structs cannot store references without a lifetime annotation.
    //id2: &str,
    r#ref: String,
    display_name: String,
    email: String,
}

struct UserWithLifetimes<'a> {
    id: &'a str,
    r#ref: &'a str,
    display_name: &'a str,
    email: &'a str,
}

// -----------------------------------------------------------------------------

// Example of a struct with a Debug trait. This is used for printing using "{:?}" or "{:#?}".
#[derive(Debug)]
struct Position {
    // Debug example.
    x: f32,
    y: f32,
    z: f32,
}

// -----------------------------------------------------------------------------

// Unit structs represent a type with no fields. They can represent roles
// for example.
struct Admin;
struct User;
struct Guest;

// Function that requires specific permissions based on the role
fn access_control<T>(_role: T)
where
    T: PermissionMarker,
{
    println!("Access granted for {:?}", std::any::type_name::<T>());
}

// Trait to enforce roles
trait PermissionMarker {}

impl PermissionMarker for Admin {}
impl PermissionMarker for User {}
impl PermissionMarker for Guest {}

fn _test() {
    access_control(Admin);
    access_control(User);
    access_control(Guest);
}

// -----------------------------------------------------------------------------

trait Shape {
    fn area(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

struct Circle {
    radius: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }
}
