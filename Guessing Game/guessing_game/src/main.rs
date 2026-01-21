//There is a default set of included libraries called the "Prelude" with each new rust project
//Like C, IO is included in the "std" standard library, which is part of Prelude
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game.");
    
    //"let" = statement for declaring variable, immutable by default. "mut" option changes to muteable
    //"String" is type provided by std library. "::" denotes "new" as associated function of the "String" type. IIRC called a Constructor in C.
    //let mut guess = String::new();                      

    //Rust is a strong typed language, but it has type inference.
    //In this case it has infered that "secret_number" is an integer (i32 to be specific, though it will auto-scale accordingly)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Enter an integer guess between 1-100:");

        let mut guess = String::new(); //Had to move this down here, otherwise program crashes on second loop due to type mismatch.
                                       //Should also be noted this will not be caught by compiler.

        io::stdin()                                         
            .read_line(&mut guess)  //References immutable by default so need to denote "&mut" to make reference ammendable
            .expect("Failed to read line!"); //Rust will warn you if you don't include expect where necessary (handle your issues dude)

        //typecasting here, can shadow variables to reduce duplicate garbage
        //.trim cut whitespace from string
        //.parse converts string to annotated type
        let guess: u32 = match guess.trim().parse(){ //expression "u32": rust calls type annotation, assuming this has to do with type-inferencing
            Ok(num) => num,
            Err(_) => continue, //"_" catchall input. "continue" moves to next loop iteration ignoring all following loop code until next iteration 
        };

        println!("You guessed: {guess}"); //See below main for further example of inline variable inclusion

        //This is a match condition, it lets you branch code via arms. (Spaghetti?)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less than."),
            Ordering::Greater => println!("Greater than."), //These three statements are the arms, they "match" a value and execute the indicated code
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            }
        }
    }
}

/*
Further inline text example:
let x = 5;
let y = 10;

println!("X = {x} and Y+2 = {}", y + 2);

Output: X = 5 and Y+2 = 12
*/