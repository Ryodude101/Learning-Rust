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
        active: true,
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