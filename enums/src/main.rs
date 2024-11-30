// -----------------------------------------------------------------------------
// Enums
// -----------------------------------------------------------------------------

fn main() {
    let shape1 = PrimitiveShape2D::Polygon {
        vertices: vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)],
    };
    let shape2 = PrimitiveShape2D::Circle { radius: 10.0 };
    let shape3 = PrimitiveShape2D::Rectangle {
        width: 15.0,
        height: 20.0,
    };

    let some_number: Option<f32> = Some(22.0);
    let some_absent_number: Option<f32> = None;
    let some_absent_number_2: Option<f32> = Option::None;
    println!("{}", some_number.unwrap());

    fn plus_one(x: Option<f32>) -> Option<f32> {
        match x {
            None => None,
            Some(i) => Some(i + 1.0),
        }
    }

    println!("Plus one: {}", plus_one(some_number).unwrap());

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        use Coin::*;
        match coin {
            Penny => 1,
            Nickel => 5,
            Quarter(state) => {
                println!("State quarter from {state:?} {:?}!", state);
                25
            }
            _ => 10, // Dime
        }
    }

    let coin = Coin::Quarter(UsState::Alaska);
    println!("The value in cents: {}", value_in_cents(&coin));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // This is a shoter way of writing the above match statement.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}

enum PrimitiveShape2D {
    Polygon { vertices: Vec<(f32, f32)> },
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

enum CreatureAction {
    Move { x: f32, y: f32 },
    Attack(Creature),
    Die,
    Mate(Creature),
}

trait Food {}

struct Creature {}

impl Food for Creature {}
