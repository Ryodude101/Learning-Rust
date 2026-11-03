/*
    Program to calculate the area of a rectangle
    Ryan Colòn
    11-1-26
*/

#[derive(Debug)] // This is how to implement an outer trait
struct Rectangle {
    width: u32,
    height: u32
}
// In this case we are adding the trait debug so we can get a debug printout of the Rect struct in println!

// Re-implementing "area" function using method syntax
impl Rectangle {
    fn area(&self) -> u32 { // First parameter must always be self
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    // The following function is in the impl block but does not begin with the self parameter.
    // That makes this function an Associated Function rather than a method.
    // Instead of being called with the . notation, you would call this function via Rectangle::square(u32);
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
    // In the case of this function: 'Self' is just an alias for the type that is associated with the impl block
}
// the `impl` scope associates all of the included with the 'Rectangle' type
// All of the functions do not have to be included in one impl block. It's not necessary here but
// a second "impl Rectangle {}" block could be located elsewhere and have function definitions and
// that would be valid syntax.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!(
        "The area of the rectangle is: {}",
        rect1.area()
    );

    println!("The struct rect1 is {:?}", rect1); // Print out rect1 struct, requires Debug trait
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/* 
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
*/