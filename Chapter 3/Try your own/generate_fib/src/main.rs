/*
    The goal of this program is to display the nth fibonacci number

    NO-RISC
    2.2.26
*/

use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Generate fibonacci number! Enter the index:");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input!");

        let index: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(e_code) => {println!("{e_code}"); continue;},
        };

        if index < 1 {
            println!("Please enter an index greater than 0.");
            continue;
        }

        println!("The fibonacci number is: {}", gen_fib(index));
        break;
    }
}

/*
The fibonacci sequence is the sum of the previous two terms
0, 1, 1, 2, 3, 5, 8...

In this case 5 is the 5th term
*/
fn gen_fib (index: u32) -> u64 {
    if index == 1 { 
        return 1;
    }
    
    let mut fibo_seq: [u64; 3]  = [0, 1, 1];
    for _number in 2..(index+1) {
        fibo_seq[2] = fibo_seq[1] + fibo_seq[0];
        fibo_seq[0] = fibo_seq[1];
        fibo_seq[1] = fibo_seq[2];
    }

    return fibo_seq[2];
}