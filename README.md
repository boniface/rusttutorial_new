# Rust Tutorial 
Items to be covered 

1. Chap 3: Fundamental Types 
2. Chap 4: Ownership and Moves
3. Chap 5: References 
4. Chap 7: Error Handling
5. chap 11: Traits and Generics
6. chap 13: Traits Utility
7. chap 14: Closures
8. chap 15: Iterators 
9. chap 16: Collections 
10. chap 17: Strings and Text 
11. chap 19: Concurrency 
12. chap 20: Asynchronous Programming


# Fundamental Types 

This section covers Rust’s fundamental types for representing values. If there’s often only one type that will work for 
a given variable or expression Rust lets you leave out, or elide, the type. This is called type inference.

## Fixed Width Types 
 Is a collection of fixed-width numeric types, chosen to match the types that almost all modern processors implement 
 directly in hardware.
 Fixed-width numeric types can overflow or lose precision,but are faster
 Arbitrary-precision integers and exact rationals can be found in a crate called num

| Size(bits) | Unsigned Integer | Signed Integer | Floating-point |
|------------|------------------| ---------------| ---------------|
| 8          | u8               | i8             |                |
| 16          | u16               | i16          |                |
| 32          | u32               | i32          |  f32             |
| 64          | u64               | i64         |    f64          |
| 128         | u128               | i128       |                |
| Machine word| usize              | isize      |                |


## Pointer Types 
## Arrays. Vectors and Slices 
## String Types 