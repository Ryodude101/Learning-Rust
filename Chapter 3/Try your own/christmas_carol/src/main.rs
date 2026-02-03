/*
    The goal of this program is to output the lyrics to the 12 days of christmas

    NO-RISC
    2.2.26
*/

const NUM_DAYS: usize = 12;
const GIFTS_ARRAY: [&str; NUM_DAYS] = [
    "A partridge in a pear tree",
    "Two turtle doves and", 
    "Three french hens", 
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"   
];
const DAY_ARRAY: [&str; NUM_DAYS] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];


fn main() {
    for day in 0..NUM_DAYS {
        println!("On the {} day of Christmas, my true love gave to me", DAY_ARRAY[day]);

        for i in (0..(day+1)).rev() {
            println!("{}", GIFTS_ARRAY[i]);
        }
    }
}
