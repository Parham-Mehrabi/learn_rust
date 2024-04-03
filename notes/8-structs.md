# structs
basically classes in python

```rust

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 
// we can create instances like this
// 
let user_0 = User{ 
    email: String::from("test@test.com"),
    active: true,
    username: String::from("test_username"),
    sign_in_count: 1,
};
println!("{}", user_0.email);



// 
// if there is a function for creating user we can use ``Field Init Shorthand`` like this:
// 

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
// here instead of saying ``email: email`` we just say email because:
// the email Field and the email parameter have the same name

let user_1 = build_user(String::from("email"), String::from("username"));
println!("{}", user_1.email);

// 
// creating instances from other instances:
// 

// the manual approach:
// let user_2 = User {
//     email: String::from("another@example.com"),
//     active: user_0.active,
//     username: user_0.username,
//     sign_in_count: user_0.sign_in_count,
// };

// the struct update syntax:
let user_2 = User{
    email: String::from("abdol"),
    ..user_0
};
println!("{}{}{}", user_2.username , user_2.active, user_2.sign_in_count);
// but keep it in mind we use the string username from user_0 so the pointer moved here
// println!("{}", user_0.username)
// so above code will give us an error




//
// we can also use struct to create a struct tuple which is basically a costume Type:
//

struct Color(i32, i32, i32);
struct RGB(i32, i32, i32);

let _black = Color(0, 0, 0);
let _origin = RGB(0, 0, 0);



// 
// structs can also have no values:
// 
struct AlwaysEqual;
let _subject =  AlwaysEqual;


//  Rust's borrow checker will track ownership permissions at both the struct-level and field-level.
//  For example,
// if we borrow a field x of a Point structure, then both p and p.x temporarily lose their permissions (but not p.y):

struct Point { x: i32, y: i32 }

let mut p = Point { x: 0, y: 0 };
let x = &mut p.x;               // p and p.x lose their RWO permissions here
// println!("{}", p.x);                   // you cant use p or p.x
println!("{}", p.y);                      // however p.y is free
*x += 1;                                  // both regain their permissions here
println!("{}, {}", p.x, p.y);


// example_2 trying to use p when p.x is in use
fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

let mut p = Point { x: 0, y: 0 };
let x = &mut p.x;
// print_point(&p);                     // we cant use p (just p.y is free not p and p.x)
*x += 1;
print_point(&p);


// however its ok to do this:

let mut p = Point { x: 1, y: 2 };
let x = &mut p.x;
let y = &mut p.y;
*x += 1;
*y += 1;
println!("{} {}", p.x, p.y);        // will print 2 3


```
