# learn-rust

## Raw Notes:

Systems programming is the practice of writing system software, which is software designed to provide services to other software. (the operating system, for example)

a signed integer is a number that can be positive or negative [-2^31, 2^31 - 1]

an unsigned integer is a number that can only be positive [0, 2^32 - 1] (double the size of a signed integer)

i32 - [-2^31, 2^31 - 1] (int)
i64 - [-2^63, 2^63 - 1] (long)
u32 - [0, 2^32 - 1] (unsigned int)
u64 - [0, 2^64 - 1] (unsigned long)

C and C++ are dangerous because they allow you to access and manipulate memory directly, which are not caught at compile time. Compiler does not enforce any rules. The responsibility is on the programmer to write safe code.

Rust compiler enforces these rules. If a program passes the compiler's checks, it is free of undefined behavior.

The same memory safety guarantees also ensure thread safety. You can share data freely between threads as long as it is immutable.

! -> this character is used to denote a macro in Rust.

If a function body ends with an expression that is not followed by a semicolon, that's the functions return value.

In fact any block surrounded by curly braces is an expression. The last expression in the block is the return value.

`return` statements are only for explicit early returns from the midst of a function.
