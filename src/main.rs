

// const BAD_CONST = 100;       you must always annotate the type.
// const bad_variable = 100;    you cant declare a 'let' in global scope
const CONFIG:&str = "you can create global scoped const";


fn main() {
    const A: u16 = 60 * 10;
    println!("A calculated in compile time: {A}");


    let mut b: u32 = 60 * 10;
    b += 1;
    println!("b after increment: {b}");


    let x = 5;           // x is not mutable
    let x = x + 1;      // shadowing x
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    

    println!("The config is: {CONFIG}");
    
    
    
    let spaces = "   ";
    let spaces = spaces.len();      // you can even change the type using shadowing
    
    // let mut spaces2 = "   ";            // you can NOT change the type using mutation
    // spaces2 = spaces2.len();            


    println!("spaces: {spaces}");
    
}
