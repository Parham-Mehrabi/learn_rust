# the slice type

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it is a non-owning pointer.


slices are special type of references because they are "fat" pointers, which means they have metadata

we use range syntax to create slices, we can specify ```[start..end]```, if we remove either of them it means the max value for that,
for example ```[..4]``` means from start to 4 and ```[2..]``` means from 2 to end


string literals are also slices

```rust
let s = "this is a string literal";
//  it’s a slice pointing to that specific point of the binary.
```


String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);

```
this slice has type ``&[i32]`` and works the same way as string slices
