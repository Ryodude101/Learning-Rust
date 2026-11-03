# Enums and Pattern Matching
Where structs are used to tie related fields and data, enums are a way of saying a value is one of a possible set of values. An example would be that a `Rectangle` is a type of shape, that also includes `Circle` and `Triangle`. You can encode these possibilities as an enum.

**Declarations and Instantiation:**
```Rust
// Declaration, There are two kinds of IP address: V4 and V6
enum IpAddrKind {
    v4,
    v6
}

// Two new instances
let four = IpAddrKind::v4;
let six = IpAddrKing::v6;

// A function that could take either variant of IpAddrKind
fn route(ip_kind: IpAddrKind){}
```
Data can also be organized in variants like tuples, this is probably a prefered way to handle this over structs:
```Rust
enum IpAddr {
    v4(String),
    v6(String)
}

let home = IpAddr::v4(String::from("127.0.0.1"));
let loopback = IpAddr::v6(String::from("::1"));
```
variants also do not have to hold the same kinds of data, they can hold different data sets and still be valid. Functions associated with enums are done the same way they are with structs, using the `impl` keyword scope:
```Rust
// Assume enum type of Message
impl Message {
    fn call(&self) {
        // Do stuff
    }
}

let m = Message(...);
m.call(); // Use function
```

## Flow Control with Match:
This section reminds me of the switch/case statements in C. It's a very similar method of pattern matching and executing code. The example used in the textbook is that of inserting a coin into a coin machine. When the coin is inserted, it will roll over the slots until it reaches the first one that fits and fall into that bucket. Similarly, a type will be matched in sequence and a section of code (called an  arm) will run upon the first match. The neat thing about rust is patterns can be made up of many things: literal values, variable names, wildcards, etc. See an example below continuing on the US coin analogy:
```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    // In this example we will pattern match to the type of coin
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
    // '=>' Syntax is how you denote the code arm
    // The syntax can be expanded with scope braces `{}`
}
```
**important note:** Match statements are exhaustive, we must cover all cases. Thankfully the Rust compiler is smart enough to know when we leave out a case and is also smart enough to tell us sometimes what cases we forgot!

**Catch-All Pattern: Placeholder '_'**

In such cases where we might be using something like literal value comparisons, it may be impossible to account for the infinite possible values that code could compare to. Rust provides and '_' pattern for match statements that essentially for all value cases that are not handled:
```Rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // Catch-all must be last pattern
    // You would only use one of these in the real world
    other => move_player(other), // use `other` if you want to bind the value
    _ => re-roll() // use `_` if you do not care about / do not want to use the value
}
```

## Flow control with If Let
You can essentially think of `If Let` syntax as being a more concise way of only pattern matching a specific instance and just running it's arm. You can also follow on with an else statement. It's another option in the rust toolbox:
```Rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```