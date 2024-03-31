# Ownership Errors

n this section, we will discuss several case studies of common ownership errors. Each case study will present a function rejected by the compiler. Then we will explain why Rust rejects the function, and show several ways to fix it.

A common theme will be understanding whether a function is actually safe or unsafe. Rust will always reject an unsafe program  
When thinking about how to fix the error, we need to ask: __why is this program unsafe?__  


there are serval situations for __fixing an program__ we are going to explore:


1. Returning a Reference to the Stack
2. Not Enough Permissions
3. Aliasing and Mutating a Data Structure
4. Copying vs. Moving Out of a Collection
5. Mutating Different Tuple Fields
6. Mutating Different Array Elements

<br>
<br>


## Fixing an Unsafe Program: Returning a Reference to the Stack  
```rust
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```
why this program is unsafe?  
because of the __"Data Must Outlive All Of Its References"__ principle  

- one way of fixing this problem is to __move ownership__ of the string out of the function:
    ```rust
    fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
    }
    ```

- another possible fix is to return string literal which lives forever(indicated by 'static)
    ```rust
    fn return_a_string() -> &`static str {
        "hello world"
    }
    ```
    This solution applies if we never intend to change the string, and then __a heap allocation is unnecessary__ !!

- another solution is to defer borrow-checker to runtime by using garbage collection, for example you can use a "reference-counted pointer":
    ```rust
        use std::rc::Rc;
        fn return_a_string() -> Rc<String> {
            let s = Rc::new(String::from("Hello world"));
            Rc::clone(&s)
        }
    ```

- the last solution for this situation is to ask caller to provide a "slot" to put the string using mutable reference

    ```rust
        fn return_a_string(output: &mut String) {
            output.replace_range(.., "Hello world");
        }
    ```
    - With this strategy, the caller is responsible for creating space for the string
    - __This style can be verbose__, but it __can also be more memory-efficient__ if the caller needs to carefully control when allocations occur.


<br>
<br>


## Fixing an Unsafe Program: Not Enough Permissions  

Another common issue is trying to mutate read-only data, or trying to drop data behind a reference.

```rust
    fn stringify_name_with_title(name: &Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
```
This program is unsafe because .push() could invalidate other references to name outside of stringify_name_with_title, like this:
```rust
let name = vec![String::from("Ferris")];
let first = &name[0];
stringify_name_with_title(&name);
println!("{}", first);
```
In this example, a reference first to name[0] is created before calling ```stringify_name_with_title```. The function ```name.push(..)``` reallocates the contents of name, which invalidates first, causing the println to read deallocated memory.

- One straightforward solution is to change the type of name from ```&Vec<String>``` to ```&mut Vec<String>```:
    ```rust
        fn stringify_name_with_title(name: &mut Vec<String>) -> String {
            name.push(String::from("Esq."));
            let full = name.join(" ");
            full
        }
    ```
    __But this is NOT a good solution!__  
    Functions should not mutate their inputs if the caller would not expect it. maybe if it was something like ```add_title_to_name``` it would be a good approach but not here.  


- Another option is to take ownership of the name, by changing ```&Vec<String>``` to ```Vec<String>```:
    ```rust
    fn stringify_name_with_title(mut name: Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
    }
    ```
    __But this is also not a good solution!__  
    it is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String.     
    this version of ```stringify_name_with_title``` would make the input ``name`` __unusable__, which is very annoying to a caller

- the ideal solution is using ```&Vec``` and then copy it:
    ```rust 
        fn stringify_name_with_title(name: &Vec<String>) -> String {
            let mut name_clone = name.clone();
            name_clone.push(String::from("Esq."));
            let full = name_clone.join(" ");
            full
        }
    ```
    By cloning name, we are allowed to mutate the local copy of the vector. However, the clone copies every string in the input. We can avoid unnecessary copies by adding the suffix later:
    ```rust
    fn stringify_name_with_title(name: &Vec<String>) -> String {
        let mut full = name.join(" ");
        full.push_str(" Esq.");
        full
    }
    ```
    this will work because ```slice::join()``` already copies data in name into the string ```full```.  
    


<br>
<br>



## Fixing an Unsafe Program: Aliasing and Mutating a Data Structure


Another unsafe operation is using a reference to heap data that gets deallocated by another alias.
```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();


    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```

``let largest = ...`` will remove W permission on ``dst`` and ``dst.push(..)`` require W permission   

why this program is unsafe?  
Because ``dst.push(..)`` could deallocate the contents of ``dst``, invalidating the reference ``largest``.


to fix the problem we need to shorten the lifetime of ``largest`` to not overlap with ``dst.push(..)``
- one possible option is to clone ``largest``:
    ```rust
    fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
        for s in src {
            if s.len() > largest.len() {
                dst.push(s.clone());
            }
        }
    }
    ```
    however this may cause a performance hit for allocating and copying string data

- another approach is do all the length comparisons first and then mutate ``dst`` next:
    ```rust
    fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
        let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
        let to_add: Vec<String> = 
            src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
        dst.extend(to_add);
    }
    ```
    but this one result performance hit too for allocating vector ``to_add``.


- the final approach is to copy out length of ``largest``, this way we don't duplicate the data inside it, this solution is arguably the most idiomatic and the most performan:
    ```rust
    fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
        let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
        for s in src {
            if s.len() > largest_len {
                dst.push(s.clone());
            }
        }
    }
    ```


<br>
<br>


## Fixing an Unsafe Program: Copying vs. Moving Out of a Collection  
imagine this scenario:
```rust
let v: Vec<i32> = vec![0, 1, 2];
let n_ref: &i32 = &v[0];
let n: i32 = *n_ref;
```
everything is alright here, but if the vector was a vector of strings ```Vec<String>```, it would error
```rust
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref;
```
but whats the difference?
the difference is ``i32`` is a copied data-type but string is not, in other words for ``i32`` it really copy the data, but for string it just copy the pointer

but why is it unsafe?
imagine this scenario:
```rust
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref;

// These drops are normally implicit, but we've added them for clarity.
drop(s);
drop(v);
```
What happens here is a __double-free.__

After executing ``let s = *s_ref``, both ``v`` and ``s`` think they own ``"Hello world"``, so after one of them is dropped, their data will be deallocated and second free will result undefined behavior

so what can we do for vectors of non-copy types like ``String``?

- first we can avoid taking ownership of the string and just use an immutable reference:
    ```rust
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
    ```
- second we can clone it:
    ```rust
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
    ```
- the final way is to use methods like Vec::remove (which is like List.pop() in python) to move the string out of the vector:
    ```rust
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
    ```


<br>
so far we checked programs which were actually unsafe, but sometimes Rust even error us for safe programs:
<br>


## Fixing a Safe Program: Mutating Different Tuple Fields

imagine this program:
```rust
fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
```
this will works fine, because after declaring ``first`` we just take W permission on ``name.0`` so we can write on ``name.1``(it still keep the W permission) but if we use a function for ``let first = &name.0`` it will error:

```rust
fn get_first(name: &(String, String)) -> &String {
    &name.0
}
fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = get_first(&name);
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
```
technically these two programs are not really that different, and second one is actually safe too

but rust doesn't look at the implement of ```get_first()```, it just look at the signature, so it says some String is getting borrowed so it will remove W permission on both ``name.0`` and ``name.1``

until this day rust compiler is not that smart to understand this so what we can do?
- we can either use inline just as first implement
- or we can use ``cells`` that we will use in future


<br>
<br>


## Fixing a Safe Program: Mutating Different Array Elements

rust don't care which index you are working with in an array and consider it as single path ``array[_]`` 

but why rust does that? imagine a situation like this:
```rust
let idx = a_complex_function();
let x = &mut a[idx];
```
rust ain't guessing the ``idx`` so it assume that ``idx`` could be anything

in some situations like this:
```rust
let mut a = [0, 1, 2, 3];
let x = &mut a[1];
let y = &a[2];
*x += *y;
```
rust will reject this code, the reason is after declaring ``x``, ``a[_]`` will lose R permission so you cant say ``let y = &a[2]``, however its not unsafe because we are referencing to ``a[1]`` not ``a[2]``

- for issues like this Rust often provide a function in standard library that can handle it, for example in this scenario, we could use ``slice::split_at_mut``:
    ```rust
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    ```
- but how does that method work? it will use ``unsafe`` block,
it is not recommended but we could do it ourselves like this:
    ```rust
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
    ```

unsafe blocks are sometimes necessary to work around rust borrow checker limitations and we will discuss them in future
but in general we should first try to solve this kind of problems with standard libraries which use unsafe block, and try to avoid using it ourselves

