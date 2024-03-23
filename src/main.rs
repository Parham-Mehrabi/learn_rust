fn main() {

    // 
    // when variables have their own data when they use stack
    // 
    let mut my_num = 0;    
    let mut _your_num = my_num;     // it will create a real copy of it in memory
    my_num += 1;                         // only affects my_num
    println!("{_your_num}");
    _your_num += 1;                      // we still have access to _your_num even after passing it through the macro
    move_a_num(my_num);                  // even if we pass them to a function like this we still have them in memory
    println!("{my_num}");                // and we can use them



    // 
    // when variables use heap, they are just a pointer to a value (pointee)
    // 
    let my_num2 = Box::new(2);
    let _your_num2 = my_num2;
    // println!("{my_num2}");               // this will result an error since we moved the pointer to _your_num2
    println!("{_your_num2}");         
    move_a_box(_your_num2);
    // println!("{_your_num2}");            // this one will also result an error since we move _your_name_2 to a function before


    // 
    // we can deep-copy it to create another pointer and pointee using .clone()
    // 
    let my_num3 = Box::new(3);
    let _your_num3 = my_num3.clone();
    println!("{my_num3}");               
    println!("{_your_num3}");         


    // 
    // and we can also return moved value to use it again
    // 
    let mut my_num4 = Box::new(4);
    my_num4 = move_a_num_and_return(my_num4);
    println!("{my_num4}");
}


fn move_a_box(_a: Box<i32>) {
    // This space intentionally left blank
  }

fn move_a_num(_a:i32){

}
fn move_a_num_and_return(a:Box<i32>) -> Box<i32>{
    return a
}