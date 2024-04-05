#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,

}
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // this one is still an associated function but its not a method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// we could separate these by defining multiple `impl` blocks like this
// impl Rectangle {
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
// }
// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 is {:?}", rect1.area()); 
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let _sqr_1 = Rectangle::square(5);


    // Method Calls are Syntactic Sugar for Function Calls
    let mut r = Rectangle { 
        width: 1,
        height: 2
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);
    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
    

    // second approach will automatically handle count of & and * needed:
    let r = &mut Box::new(Rectangle { 
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

}
