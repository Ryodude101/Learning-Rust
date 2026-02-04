# Ownership

Seems this is going to be a really important concept in Rust so I'm making a note page to keep track of things I learn about this topic.    

As a brief forenote I started to dip into this topic without fully understanding it when writing the Christmas Carol program.   
My program wouldn't compile the constant string arrays because I was trying to use the heap method String instead of the stack method &str. 

## Rules of ownership
1. Each value in Rust has an owner.
2. There can be only **one** owner at a time.
3. When the owner goes out of scope, the value will be dropped (the memory freed).