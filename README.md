# Rust Tutorial 
Items to be covered 

1. Chap 3: Fundamental Types 
2. Chap 4: Ownership and Moves

There are two memory types management
  1. Safety first. Use garbage collector to clean up objects and no dangling pointers or references can be 
  2. Control First 






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
 directly in hardware.  Fixed-width numeric types can overflow or lose precision,but are faster
 Arbitrary-precision integers and exact rationals can be found in a crate called num

Table below shows the Rust Numeric Types

| Size(bits) | Unsigned Integer | Signed Integer | Floating-point |
|------------|------------------| ---------------| ---------------|
| 8          | u8               | i8             |                |
| 16          | u16               | i16          |                |
| 32          | u32               | i32          |  f32           |
| 64          | u64               | i64          |   f64          |
| 128         | u128               | i128        |                |
| Machine word| usize              | isize       |                |

 Machine word is a **value** the size of an address on the machine the code runs on, 32 or 64 bits.

## Integer Types

Rust’s unsigned integer types use full range to represent positive values and zero

Table 3-3. Rust unsigned integer types

| Type  | Range                      |
| ----- | ------------------------- |
| u8    | 0 to 2 <sup>8</sup> –1 (0 to 255)     |
| u16   | 0 to 2 <sup>16</sup> −1 (0 to 65,535) |
| u32   | 0 to 2 <sup>32</sup> −1 (0 to 4,294,967,295) |
| u64   | 0 to 2 <sup>64</sup> −1 (0 to 18,446,744,073,709,551,615, or 18 quintillion) |
| u128  | 0 to 2 <sup>128</sup> −1 (0 to around 3.4✕10 <sup> 38</sup> ) |
| usize | 0 to either 2 <sup>32</sup> −1 or 2 <sup>64</sup> −1 |

Rust’s signed integer types use the two’s complement representation, using the same
bit patterns as the corresponding unsigned type to cover a range of positive and negative values

| Type  | Range                      |
| ----- | ------------------------- |
| i8    | −2 <sup>7</sup> to 2 <sup>7</sup> −1 (−128 to 127)    |
| i16   | −2 <sup>15</sup> to 2 <sup>15</sup> −1 (−32,768 to 32,767) |
| i32   | −2 <sup>31</sup> to 2 <sup>31</sup> −1 (−2,147,483,648 to 2,147,483,647 |
| i64   | −2 <sup>63</sup> to 2 <sup>63</sup> −1 (−9,223,372,036,854,775,808 to 9,223,372,036,854,775,807) |
| i128  | −2 <sup>127</sup> to 2 <sup>127</sup> −1 (roughly -1.7✕10 <sup>38</sup> to +1.7✕10 <sup>38</sup> ) |
| isize | isize Either −2 <sup>31</sup> to 2 <sup>31</sup> −1, or −2 <sup>63</sup> to 2<sup> 63</sup> −1 |

* Rust uses the u8 type for byte values. 
* For example, reading data from a binary file or socket yields a stream of u8 values.
* Rust treats characters as distinct from the numeric types: 
* a char is not a u8 , nor is it a u32 (though it is 32 bits long).
* The usize and isize types precision matches the size of the address space on the target machine: they are
  32 bits long on 32-bit architectures, and 64 bits long on 64-bit architectures.
* Rust requires array **indices** to be usize values. 
* Values representing the sizes of arrays or   vectors or counts of the number of elements in some data structure also generally
  have the usize type.
* Integer literals in Rust can take a suffix indicating their type: 42u8 is a u8 value, and
  1729isize is an isize . 
* If an integer literal lacks a type suffix, Rust puts off determining its type until it finds the value being used in a way that pins it down: stored in a
  variable of a particular type, passed to a function that expects a particular type, compared with another value of 
  a particular type. 
* If  multiple types could work, Rust defaults to i32 if that is among the possibilities. Otherwise, Rust reports the ambiguity as an error.
* The prefixes 0x , 0o , and 0b designate hexadecimal, octal, and binary literals.
* To make long numbers more legible, you can insert underscores among the digits.
* For example, you can write the largest u32 value as 4_294_967_295 . 
* The exact placement of the underscores is not significant 
* you can break hexadecimal or binary numbers into groups of four digits rather than three, as in 0xffff_ffff , 
* Or set off the  type suffix from the digits, as in 127_u8 .


Integer Literal Table Examples 

| Literal  | Type     | Decimal Value |
| -------- | -------- | ------------- |
| 116i8    | i8       | 116           |
| 0xcageu32| u32       | 51966       |
| 0b0010_1010| inferred | 42 |
| 0o106     | inferred | 70|


 * Although numeric types and the char type are distinct, Rust does provide byte literals, character-like literals for 
u8 values: 
 * b'X' represents the ASCII code for the character X , as a u8 value. 
 * For example, since the ASCII code for A is 65, the literals b'A' and 65u8 are exactly equivalent. 
 * Only ASCII characters may appear in byte literals.
 * There are a few characters that you cannot simply place after the single quote,
because that would be either syntactically ambiguous or hard to read. 
 * The characters in Table 3-6 can only be written using a stand-in notation, introduced by a backslash.

| Character | Byte literal | Numeric equivalent |
|----------| ------------- | ------------------ |
| Single quote, ' | b'\ ' '   | 39u8              | 
| Backslash, \ | b' \ \ '   | 92u8              | 
| Newline | b'\n'   | 10u8              | 
| Carriage Return  | b'\r'   | 13u8              | 
| Tab | b'\t'   | 9u8              | 

* For characters that are hard to write or read, you can write their code in hexadecimal
instead. 
* A byte literal of the form b'\xHH' , where HH is any two-digit hexadecimal
number, represents the byte whose value is HH . 
* For example, you can write a byte literal for the ASCII “escape” control character as b'\x1b' , since the ASCII code for
“escape” is 27, or 1B in hexadecimal. 
* Since byte literals are just another notation for u8 values, consider whether a simple numeric literal might be more legible: 
* it probably makes sense to use b'\x1b' instead of simply 27 only when you want to emphasize that the value represents an ASCII code.

* You can convert from one integer type to another using the as operator.

```rust
assert_eq!(10_i8 as u16,10_u16); // in range
assert_eq!( 2525_u16 as i16, 2525_i16); // in range

assert_eq!( -1_i16 as i32,-1_i32); // sign-extended
assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended
```
* specify the type before you convert 
* Method calls have a higher precedence than unary prefix operators
```rust
fn ops_on_types(){
    // println!("{}", (-4).abs()); Will not compile
    println!("{}", -4_i32.abs()); // Prints -4 
    println!("{}", (-4_i32).abs()); // Print 4
    println!("{}", i32::abs(-4)); // Prints 4
}
```


### Checked, Wrapping, Saturating, and Overflowing Arithmetic
* When an integer arithmetic overflows, 
  * In debug build, Rust panics
  * In Release build, it wraps around 
    * Produces it produces the value equivalent to the  mathematically correct result modulo the range of the value.
    * Multiplication wraps to a negative number
Integer Arithmetic falls into four categories 
    * Checked operations return an Option of the result
    * Wrapping operations return the value equivalent to the mathematically correct
      result modulo the range of the value i.e  A x B = AB modulo 2 <sup>isize</sup>
    * Saturating operations return the representable value that is closest to the mathematically correct result. In other words, the result is “clamped” to the maximum
      and minimum values the type can represent:
    * Overflowing operations return a tuple (result, overflowed) , where result is  what the wrapping version of the function would return, and overflowed is a
      bool indicating whether an overflow occurred:

### 
## Pointer Types 
## Arrays. Vectors and Slices 
## String Types 