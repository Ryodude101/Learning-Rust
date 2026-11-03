# Structs  

## Defining Structs
**Type Declaration:**
```Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
**Instantiation:**
```Rust
fn main(){
    let user1 = User  {
        active: true,v
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```
**Access and Mutability:**
```Rust
fn main(){
    // Whole struct must be mutable, no partials
    let mut user1 = User { // Mut included here to make struct mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // Use dot notation for read and write access
}
```
**Build Function and Field Initiation:**
```Rust
// Function that builds and implicitely returns a new User struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Function that does the same but user Field Init shorthand.
fn build_user(email: String, username: String) -> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```
**Struct Update Instantiation:**

Sometimes we want to create an instance using the values of another instance, Rust has a syntax for making this easier to do than other languages. Here`s an example where we would want to clone all of the values from user1 to user2 but to change the email:
```Rust
// How you would do this in something like C
 let user2 = User {
    active: user1.active,
    username: user1.username,
    email = String::from("someoneelse@example.com"),
    sign_in_count: user1.sign_in_count,
 };

// An easier method to do the samething using update syntax
 let user2 = User {
    email = String::from("someoneelse@example.com"),
    ..user1 // This must come last in the list.
 };
```
An important not about the behavior here for *both* methods: In the case where not all fields have the copy trait (in this case here we are using strings in two fields, which do not have `Copy`) data is moved instead. So in this case 1 string got moved, `username`, which means `user1` is no longer valid in this scope and can't be used. However if we had specified a new string for both `username` and `email` then only the values with the `Copy` trait would have been copied (not moved) and thus `user1` would still be valid.

**Tuple Structs:**

Rust also has a type of struct called a `Tuple Struct` that essentially gives you the power to name the tuple as well as declare the specific types in the fields. This is useful when you want to make a tuple that is a separate, named type from the `tuple` type:
```Rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
Even though `Color` and `Point` share basically the same structure, their explicit typing prevents them from being used in place of each other. I.E. a function that takes an input type of `Point` will not accept an input that is of the type `Color`.

**Ownership of Struct Data:**

In the earlier `User` example, the type `String` was used so the struct itself had **ownership** over the data it was carrying. Technically this is not a requirement and instead of using `String`, references could've been included in that spot:
```Rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
```
However, this would've thrown a compiler error because now the structure references data that it does not have **ownership** of. And so the use of `lifetimes` is required, which will be discussed later in Chapter 10.