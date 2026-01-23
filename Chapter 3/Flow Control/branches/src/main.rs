fn main() {
    let number = 3;

    //if statements are limited to evaluating Boolean values only
    //No goofy stuff with integers/chars like in C LOL
    if number < 5 {
        println!("Conditon was true!");
    } else {
        println!("Condition was false!");
    }

    if number % 3 == 0 {
        println!("Number divisible by 3");
    } else if number % 2 == 0{
        println!("Number divisible by 2");
    } else if number % 1 == 0{
        println!("Number divisible by 1");
    } else {
        println!("Number aint divisible I guess LOL");
    }

    //if statements are expressions and so can be used on the right side of a "let" statement for assignment, but return values must all be of same type
    let _x = if number == 5 {5} else {0};
    println!("X = {x}");
}
