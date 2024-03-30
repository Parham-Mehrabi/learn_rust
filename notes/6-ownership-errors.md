# Ownership Errors

n this section, we will discuss several case studies of common ownership errors. Each case study will present a function rejected by the compiler. Then we will explain why Rust rejects the function, and show several ways to fix it.

A common theme will be understanding whether a function is actually safe or unsafe. Rust will always reject an unsafe program  
When thinking about how to fix the error, we need to ask: __why is this program unsafe?__  


there are serval situations for __fixing an unsafe program__ we are going to explore:


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
    