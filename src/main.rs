fn main() {
    let x = 2.000000000000001; // f64
    let y: f32 = 3.000000000000001; // f32

    println!("{x}");
    println!("{y}");
    
    let a:bool = true;
    let b = true;
    println!("{a}");
    println!("{b}");
    
    
    let c = "z";
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII
    let heart_eyed_cat = '😻';
    
    println!("{c}{z}{heart_eyed_cat}");
    
    

    // in rust: Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{:?}", tup);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{x} | {y} | {z}");
    println!("{five_hundred} | {six_point_four} | {one}");



    let ones = [1; 100];
    println!("{:?}", ones);

}
