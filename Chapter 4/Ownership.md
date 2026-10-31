# Ownership

Seems this is going to be a really important concept in Rust so I'm making a note page to keep track of things I learn about this topic.    

As a brief forenote I started to dip into this topic without fully understanding it when writing the Christmas Carol program.   
My program wouldn't compile the constant string arrays because I was trying to use the heap method String instead of the stack method &str. 

This leads to the first important note: **Whether you actually put something in the stack or heap is important in Rust, as it changes the way you need to handle the memory!** I do not yet know how similar or different this behavior is to C. 

The main purpose of the ownership feature is to help make managing data on the heap easier.

## Rules of ownership
1. Each value in Rust has an owner.
2. There can be only **one** owner at a time.
3. When the owner goes out of scope, the value will be dropped (the memory freed).

## Memory Allocation
For heap memory allocation:
- The memory must be requested from the memory allocator at runtime
- There needs to be a way to return the memory when we're done with it
The memory request in all languages is always done by the programming (example, instancing a class), it's the returning of the memory that is important. In languages with a garbage collector (python), the collector tracks and cleans memory and doesn't require human input. In languages without a collector (C/++), it is up to the programmer to free memory. And that is to be an exacting process, exactly one allocate for one free, which has been a source of bugs and headaches for a long time.

I don't know enough about garbage collectors to say whether this is similar or different to one, but Rust automatically handles memory by following the rules of ownership. When a rule of ownership is violated, the memory is freed.
- To free memory, Rust uses the special function `drop`. If not handled by the programmer, this function is inserted by Rust at the end of scope (the closing curly bracket)
- There is a C++ equivalent pattern to this called: *Resource Acquisition Is Initialization (RAII)*

## Variable Interactions
In order to maintain singular ownership, when heap data is "copied" in Rust, it is technically moved instead. See example below:
```Rust
let s1 = String::from("hello");
let s2 = s1;
```

In this case example, the heap memory string is shallow copied from s1 into s2. However, references to s1 instead of s2 will now throw an error `borrow of moved value: s1`. This solves the problem of the possible double free bug.

However, if you do want to deep copy heap data: that can be done with the `clone` method.
```Rust
let s1 = String::from("hello");
let s2 = s1.clone();
```
It's expensive as actual data is being duplicated, but this maybe necessary in some cases.

## Ownership Transfer
See the example below:
```Rust
let s = String::from("hello");
takes_ownership(s);

let x = 5:
makes_copy(x); //included to show that this behavior does not apply to copy trait types
```
Trying to use `s` after passing it to the function `takes_ownership` will throw an error because when it is passed to the function: the function takes ownership. This changes the scope of `s` and so when the function ends, it's memory would be freed. 

Ownership can also be transfered out of a function by a `return`, though this works as a move:
```Rust
let s1 = gives_ownership();
let s2 = String::from("hello");
let s3 = takes_and_gives_back(s2);
```
This song and dance can be simplified by using references `&s1` and the dereference operator `*` instead of moving the variables around constantly back and forth. Referencing in Rust works as it does in C. It's to pass the variable around without transferring ownership.

By default references are immutable, however mutable references (only to mutable objects) can be made by `&mut` instead of `&`. However, Rust only allows for a mutable reference to be borrowed exactly once to prevent data race conditions:
```Rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{r1}, {r2}");
```
This will throw a compile time error `cannot borrow s as mutable more than once at a time`. It is also not allowed to have both immutable and mutable references in the same scope `cannot borrow s as mutable because it is already borrowed as immutable`.
- Small correction: immutable and mutable references can exist in the same scope **only** if the immutable references are no longer used by the time the mutable reference is made. Rust will automatically clean up the immutable references and allow the mutable reference to instantiate.

Rust will is also able to detect and prevent dangling references at compile time. The book references lifetimes at this point but does not explain them, they will be covered in Chapter 10.

## Slices
You can track a portion of data out of certain data types with the following syntax:
```Rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
//You can drop one of the specifiers if it is an edge
let slice = &s[..5]; 
let slice2 = &s[6..];
let wholeslice = &s[..];
```
Internally, the slice structure is just storing a pointer to the starting index and a length value. This is a good way to handle data like this because it also allows the compiler to track the reference.