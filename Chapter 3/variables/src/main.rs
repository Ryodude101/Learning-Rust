//constants can be declared with "const" and must be typed (no inferencing)

//Shadowing variables is limited to the scope of the shadowed variables declaration
/*
let x = 6;
    loop{
        let x = x * 2;
        println!("X = {x}"); "X = 12"
    }
println!("X = {x}"); "X = 6"
*/
//Shadowing instead making a variable muteable allows us to change a variables type while preserving namesake

/*
Compound types (tuples and arrays) are fixed size only in Rust (like C)
tuples can have multipe value types (arrays can only have one)
and tuples are accessed like tuple.index; arrays like array[index]

and they begin at 0 like REAL LANGUAGES DO!!!
*/

fn main() {
    let mut x = 5;
    println!("X = {x}");
    x = 6;
    println!("X = {x}");

    let _a = [0; 5]; // Initialize array of size 5 with value 0
    let _b: [i16; 3] = [0, 0, 0]; //Initialize array of size 3 with type i16, default value zero
}
