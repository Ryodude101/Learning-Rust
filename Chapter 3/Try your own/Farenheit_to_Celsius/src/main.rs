/*
    The goal of this program is to convert a given temperature (in Farenheit)
    to Celsius.

    NO-RISC
    2.2.26
*/

use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!{"Please enter the temperature in Farenheit:"};
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let temp_f: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e_code) => {println!("{e_code}"); continue;}, //neat way I divined to print error details lol, hell yeah
        };

        let temp_c = convert_from_f_to_c(temp_f);

        println!("The temperature in Celsius = {temp_c} degC");
        break;
    }
}

fn convert_from_f_to_c(temp_f: f32) -> f32{
    (temp_f - 32.0) * (5.0/9.0)
}