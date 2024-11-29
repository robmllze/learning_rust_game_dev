// -----------------------------------------------------------------------------
// Slices
// -----------------------------------------------------------------------------

fn main() {
    let mut sentence = String::from("Hello World!!!");
    let index = first_word_end_index(&sentence);
    println!("First word index: {}", index);

    // Slices are references to portions.
    let word = &sentence[0..index];
    println!("First word: {}", word);

    let slice = first_word(&sentence);
    println!("First word: {}", slice);
    sentence.clear();

    // String literals are slices.
    let some_string = String::from("Hello world, how are you today?");
    let _hello = &some_string[..5];
    let _world = &some_string[6..11];
    let _today = &some_string[25..];

    // This is a reference to a string literal stored in the binary!
    let _literal = "Hello world, what's up?";

    let _a = [1, 2, 3, 4, 5];
    let b = [..3];
    println!("{}", b.len());
 }

fn first_word_end_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    // enumerate() returns a tuple with the index and a reference to the item.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Return the slice from the start to i.
            return &s[..i];
        }
    }
    // Convert String to a string slice.
    &s[..]
}
