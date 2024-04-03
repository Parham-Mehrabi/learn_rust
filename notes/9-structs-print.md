```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);                           // use Class.__str__
    println!("rect1 is {:#?}", rect1);                          // same thing but with a nicer format (its json in the end)
    println!("{}x{} {:?}", rect1.height, rect1.width, rect1);   // we can still use rect1    
    dbg!(rect1);                                                // print the line and file and also take ownership of the input
    // println!("{}x{} {:?}", rect1.height, rect1.width, rect1);// we can NOT user rect1 after passing it to dbg! macro    
    
}
```