//rust does not care about function declaration placement in code.
//Only that it can be seen in the scope of the caller and also not interrupt the current scope.

/*
Lines of code, and functions, are denoted by "Statements" and "Expressions"
Statements do not return values, expressions do.

This is important to note as unlike C, many lines like variable declarations are statements in rust
and therefore cannot be nested. They do not return a value when executed. I.E.:
let x = (let y = 5); will not compile

Macros, Functions, and Scope brackets "{}" are expressions

let y = {
    let x = 6;
    x + 1
};
    results with y = 7

    Note the final line in the scope expression does not have a closing ;. A semicolon would convert the expression to a statement
*/

fn main() {
    println!("Hello, world!");

    another_function(fives(), 'h');
}

//function declarations must declare the type of each parameter, can not be inferred
fn another_function(x: i32, y: char) {
    println!("The value of x, y is {x}, {y}");
}

//How you denote a function that returns a value
//Return value does not need to be named but its type has to be declared after the (->)
fn fives() -> i32 {
    5
}
//Functions can infer the return value, but you can also return with the "return" keyword and a value (like C)