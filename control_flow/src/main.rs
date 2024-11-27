// -----------------------------------------------------------------------------
// Control Flow
// -----------------------------------------------------------------------------

fn main() {
    // Just a loop.
    let mut n = 0;
    loop {
        n += 1;
        println!("m -> {}", n);
        if n == 10 {
            break;
        }
    }

    // A loop with labels.
    let mut x = 0;
    let mut y = 0;
    'x_loop: loop {
        x += 1;
        y %= x;
        'y_loop: loop {
            y += 1;
            x += 1;
            if x == 20 {
                break 'x_loop;
            }
            if y == 3 {
                break 'y_loop;
            }
        }
    }
    println!("(x, y) = ({}, {})", x, y);

    // A for loop.
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for e in numbers.iter().rev() {
        if *e == 0 {
            println!("LIFT OFF!!!");
        } else {
            println!("T-{}", e);
        }
    }

    const MORE_NUMBERS: [char; 26] = make_char_array();
    for e in MORE_NUMBERS {
        println!("Pig says {}", e.to_uppercase());
    }
}

// -----------------------------------------------------------------------------

const fn make_char_array() -> [char; 26] {
    let mut arr = ['a'; 26];
    let mut i = 0;
    while i < 26 {
        arr[i] = (b'a' + i as u8) as char;
        i += 1;
    }
    arr
}
