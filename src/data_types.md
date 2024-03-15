
# Data Types

1. Rust’s __char__ type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII

2. in Rust: __Tuples__ have a fixed length: once declared, they cannot grow or shrink in size.

3. in Rust, __tuples__ indies are accessible using a period(.) instead of brackets

4. Unlike __arrays__ in some other languages, arrays in Rust have a fixed length.

5. An __array__ isn’t as flexible as the __vector__ type, though. A __vector__ is a similar collection type provided by the standard library that is allowed to grow or shrink in size

6. You write an __array__’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the __array__, like so: 

```
let my_array: [i32;5] = [1, 2, 3, 4, 5]; 
```
and you can also use this syntax to initialize an array with same elements

```
let ones: [1; 100];
```

7. to print an __array__ or a __tuple__ you can use this syntax:
```
let zeroes_array[1;5];
println!("{:?}", zeroes_array);
-----------------------------
// output ~> [0, 0, 0, 0, 0]
```
```
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("{:?}", tup);

-----------------------------
// output ~> (500, 6.4, 1)
```