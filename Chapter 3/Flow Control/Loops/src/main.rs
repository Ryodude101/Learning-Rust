fn main() {
    loops();
    conditionals();
    fours();
}

fn loops() {
    let mut count = 0;

    'counting_up: loop {  //Loops can be labeled, labels must start with a single quote character
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //single quote must be used even when calling
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}

fn conditionals() {
    let mut number = 3;

    while number != 0 { //Pretty simple setup reminds me of C
        println!("{number}");

        number -= 1;
    }

    println!("Liftoff!");
}

fn fours() {
    let a = [10, 20, 30, 40, 50];

    for element in a {  //Interesting departure from C conventions, reminds me more of Python. Possibly to limit programmer error, respectable.
        println!{"Value = {element}"};
    }

    for number in (1..4).rev(){ //Yes it is more nice lol
        println!("{number}");
    }
    println!("Liftoff!");
}