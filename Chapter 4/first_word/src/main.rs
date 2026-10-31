/*
First Word  Finder
Ryan Colòn
10-31-2026
*/

fn main() {
    let s = String::from("Hello, world!");

    let fw = first_word(&s);

    println!("{}", fw);
}

// Function takes a string and returns just the first word (or the whole string if there are no spaces)
// Works by comparing byte literals, so only ASCII valid here not UTF-8
fn first_word(s: &str) -> &str{
    let sbytes = s.as_bytes();

    // A lot going on here to be expanded upon in Chapter 8
    for (i, &item) in sbytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    return &s[..];
}