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


}
