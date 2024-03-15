fn main() {
    println!("{:?}", my_func());
    println!("{:?}", my_func3());
    println!("{:?}", my_func4());
    println!("{:?}", my_func5());

}

fn my_func() -> i32{
    1
}


// fn my_func2() -> i32{
//     2;
// }
//  this one will return () not i32

fn my_func3() -> i32{
    return 3;
}

fn my_func4(){
    4;
}

// fn my_func4() -> (){         // this one will give us an error because we are returning 4
//     4
// }
// fn my_func4() -> (){         // this one is ok since we are not returning anything and by default it will return ()
//     4;
// }

fn my_func5(){
    return ()
}
