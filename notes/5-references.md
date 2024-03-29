# references


there will be some key points that im just going to mention here and explore more in rust-code-examples,  
you can either roll-back to the commit i created this to checkout main.rs,
or use the code snippet at the end of the file


1. references are not-owning pointers
    - 
2. Dereferencing a Pointer Accesses Its Data
    - 
    - you can use * to dereference a reference
3. Avoids Simultaneous Aliasing and Mutation
    - 
    - data should never be aliased and mutated at the same time.

4. References Change Permissions on Paths
    - 
    - The core idea behind the borrow checker is that variables have three kinds of __permissions__ on their data:
        - write(W) -> data can be __copied__ to another location.
        - read(R) ->  data can be __mutated__ in-place.
        - own(O) ->  data can be __moved__ or __dropped__.

5. The Borrow Checker Finds Permission Violations
    - 
    - The borrow checker looks for potentially unsafe involving references
    - Rust uses these permissions in its borrow checker.

6. Mutable References Provide Unique and Non-Owning Access to Data
    - 
    ```rust 
    // there is two main difference between mutable and immutable references:
    //
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    v.push(4);
    // above num is immutable but below is mutable
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
    // compared to when num was immutable 
    // 1. when num was immutable reference, v still had R permission 
    //      now that num is mutable reference, v has no permission at all ---
    // 
    // 2. when num was immutable reference, the path *num had only R permission
    //      now that num is mutable reference, *num also gain W permission too
    // 
    //                        v   ->  ---                                                    v   ->  R--
    // it is now like this:   num ->  RO-     but when it was mutable reference, it was:     num ->  RO-
    //                       *num ->  RW-                                                   *num ->  R--  
    // 
    ```
    - When num was an immutable reference, v still had the R permission. but later that num is a mutable reference, v has lost all permissions while num is in use.
    - When num was an immutable reference, the path *num only had the R permission. but later that num is a mutable reference, *num has also gained the W permission.

7. Permissions Are Returned At The End of a Reference's Lifetime
    - 
    - after the last time you use a reference, the permissions of referenced path, will return


8. Data Must Outlive All Of Its References
    - its clear, you can reference something that doesn't exist
    1. you can't drop a data with alive reference to it
    2. in need F permission, and for that rust need to know which reference belong to which path (the example of function with two arguments where return a reference)
        ```rust
        fn first_or(first_str: &Vec<String>, second_str: &String) -> &String {
            if first_str.len() > 0 {
                &first_str[0]
            } else {
                second_str
            }
        } 
        ```
        this function is unsafe because rust don't know the returned value is a reference of first_str or second_str
    
    3. another example is when you create a variable inside a function and return a reference to it, 
    the __reference which is returned will outlive its data__, since the variable will drop after the execution of the function


<br>
<hr/>
<br>
<br>
<br>

```rust
fn main() {

    // 
    // referencing and dereferencing using & and *
    // 
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                             //     so x points to the value 2
    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value
    
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it
    // 
    // in this scenario:
    //      x points to heap
    //      value of stack increased to 2
    //      a has the value of 1 in Stack
    //      r1 points to x
    //      b has value of 2 in stack
    //      r2 points to heap
    //      c has the value of 2 in stack
    println!("x={x}, a={a}, r1={r1}, b={b}, c={c}");



    // 
    // there is 2 ways for using a reference either using dot method or calling function and dereferencing it
    // 
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);        // explicit dereference
    let x_abs2 = x.abs();                  // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    
    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);      // explicit dereference (twice)
    let r_abs2 = r.abs();                 // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);
    
    let s = String::from("Hello");
    let s_len1 = str::len(&s);    // explicit reference
    let s_len2 = s.len();              // implicit reference
    assert_eq!(s_len1, s_len2);

    // the thing is when you use dot method, it doesn't matter how many jumps we need to reach pointee



    let x = Box::new(0);
    let y = Box::new(&x);
    let z = 1 + ***y;
    assert_eq!(z, 1);
    // in this example we used 3 asterisks because:
    // first one will give us the pointee of y which is a pointer to x
    // second one will give us the pointer we used to point the x 
    // last one will give us actual 0 which is pointed by x




    // 
    // after modifying a vector like below, the pointee will completely change, because vector has a certain length and cap in memory
    // so after pushing a new item to it, we need to allocate a new heap with new cap and length
    // so in this case older pointers to previous vector will have undefined behavior so rust prevent us from modifying the vector after we borrow it
    // 
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);                              // we can modify it before borrowing it
    let num: &i32 = &v[2];                  // here we borrow v to collect its third index
    // v.push(4);                           // but now we can not modify v anymore because of previous line
    println!("Third element is {}", *num);
    // Pointer Safety Principle: data should never be aliased and mutated at the same time.




    // 
    // by default variables' permission is RO(Read Own), and when we add mut after let it become RWO(Read Write Own)
    // when we reference it it become R (-O -W) until we use the reference
    // 
    let mut v: Vec<i32> = vec![1, 2, 3];
    // v.push(4)                                // this one is ok
    let num: &i32 = &v[2];
    // v.push(4);                               // this one is NOT ok
    println!("Third element is {}", *num);
    v.push(4);                                  // this one is ok too
    // 
    // 1. after let mut v = (...) the variable v is initialized with RWO
    // 
    // 2. after let num: &i32 = &v[2], the data in v has **BORROWED** by num and this three will happened:
    //      1. the borrow remove O and W from v and v's permission become R
    //      2. the variable num has gained RO permission (it doesn't have W since it doesn't have mut after let)
    //      3. the `path`, *num has gained R permission
    // 
    //      v   ->  R
    //      num ->  RO  
    //     *num ->  R
    // 
    //  3. after println!(...), the variable num is no longer in use, so v is no longer borrowed, therefore:
    //     v will regain WO permission too again, num and *num will lose all their permissions too
    //   
    //      v   ->  RWO
    //      num ->  --- 
    //     *num ->  ---
    //   
    // keep it in mind it doesn't mean we cant use them multiple times, for example here we could have another println! just after the first one
    // in that case num simply loses its permissions later
    //



    // 
    // when we put a 'mut' after let for the one who is borrowing, the mut means we can assign variable to a different reference
    // but its value is mutable unless we use mut& instead of mut
    // More generally, permissions are defined on paths and not just variables
    // 
    let x = 0;
    let mut y = 1;

    let mut x_ref = &x;
    
    println!("x_rf = {x_ref}");
    x_ref = &y;             // we can change the reference since x_ref itself is mutable
    // *x_ref += 1;         // this one will error since value of x_ref is not mutable (its & not &mut)
    println!("x_rf = {x_ref}");

    let x_ref_2 = &mut y;
    *x_ref_2 += 1;          // we can modify the value of pointee this way since this time its mutable
    // x_ref_2 = &mut x;    // this one will error too since x_ref_2 itself is not mutable (unlike x_ref)
    println!("x_ref_2={x_ref_2}");
    println!("y={y}");
    // 
    // references using & are called immutable references (AKA shared references)
    // references using &mut are called mutable references (AKA unique references)
    // 



    //
    // there is two main difference between mutable and immutable references:
    //
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
    // compared to when num was immutable (2 examples ago)
    // 1. when num was immutable reference, v still had R permission 
    //      now that num is mutable reference, v has no permission at all ---
    // 
    // 2. when num was immutable reference, the path *num had only R permission
    //      now that num is mutable reference, *num also gain W permission too
    // 
    //                        v   ->  ---                                                    v   ->  R--
    // it is now like this:   num ->  RO-     but when it was mutable reference, it was:     num ->  RO-
    //                       *num ->  RW-                                                   *num ->  R--  
    //    



    // 
    // reference's lifetime
    //

    // so far we saw when a permissions(RWO), will return after the last time we use the reference, and its called the reference's lifetime,
    //  however, a reference can have multiple branch lifetime for example like variable c in below function:
    fn _ascii_capitalize(v: &mut Vec<char>) {
        let c = &v[0];

        if c.is_ascii_lowercase() {

            let up = c.to_ascii_uppercase();
            v[0] = up;
            
        } else {
            println!("Already capitalized: {:?}", v);
        }
    }
    // 
    // The variable c has a different lifetime in each branch of the if-statement. In the then-block,
    //  c is used in the expression c.to_ascii_uppercase(). Therefore *v does not regain the W permission until after that line.
    // However, in the else-block, c is not used. *v immediately regains the W permission on entry to the else-block.
    // 



    // 
    // sometimes references are declare in other scopes but the rules are the same:
    // 
    fn get_first(v: &Vec<i32>) -> &i32 {
        &v[0]
    }
    let mut nums = vec![1];
    let first_num = get_first(&nums);
    // first_num.push(2);                        // this will error since variable `first_num` is still alive here
    println!("{first_num}");
    nums.push(2);                               // but this one is ok since first_num's lifetime is ended here
    




    // 
    // "data must outlive any references to it" and "flow(F)" permission
    // 
    // when its not in a function, its so simple like this:

    let s = String::from("Hello world");
    let s_ref = &s;
    // drop(s);                 // we cant drop the `s` here since s_ref is still alive
    println!("{}", s_ref);
    drop(s);                    // but now its safe to drop `s`

    // but when its in a function it may be a little tricky,
    // consider 2 examples ago where the reference was created in get_first function:
    // 
    // fn get_first(v: &Vec<i32>) -> &i32 {
    //     &v[0]
    // }
    // let mut nums = vec![1];
    // let first_num = get_first(&nums);
    // first_num.push(2);                        // this will error since variable `first_num` is still alive here
    // println!("{first_num}");
    // nums.push(2); 
    // 
    // here the returned value clearly is a reference from v so it have the F permission
    // The F permission is expected whenever an expression uses an input reference, or returns an output reference.
    // Unlike the RWO permissions, F does not change throughout the body of a function.
    // A reference has the F permission if it's allowed to be used (that is, to flow) in a particular expression.
    // but sometimes we don't have the permission like this:

    // fn first_or(first_str: &Vec<String>, second_str: &String) -> &String {
    //     if first_str.len() > 0 {
    //         &first_str[0]
    //     } else {
    //         second_str
    //     }
    // }
    // this function will not compile because the returned value has no F permission,
    // why? because the returned value is reference but its not clear if it is a reference to first_str or second_str
    // why does it matter? imagine this scenario:

    // let first_str = vec![];
    // let second_str = String::from("default");
    // let s = first_or(&first_str, &second_str);
    // drop(second_str);
    // println!("{}", s);

    // here rust don't know if it is safe to let us drop second_str or not since we don't know what kinda of reference `s` is



    // 
    // the last example we are going to check is again about "data must outlive any references to it" principle
    // 

    // fn return_a_string() -> &String {
    //     let s = String::from("Hello world");
    //     let s_ref = &s;
    //     s_ref
    // }
    // 
    // missing lifetime specifier
    // 
    // this one will error too for an obvious reason; the data must outlive any references to it,
    // after the function is executed there is no s anymore but s_ref is returned and will remain, so we are referencing to something doesn't exist

}

```

