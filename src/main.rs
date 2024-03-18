fn main() {
    //
    //  returning value from loop:
    //
    let mut counter = 0;
    let result = loop {
        // you can use 'loop' to declare a variable
        counter += 1;

        if counter == 10 {
            break "we can return values from loops"; // this string will be the value of 'result' variable
        }
    };
    println!("The result is {result}");
    println!("The counter is {counter}");

    //
    //  using loop label to specify which loop is the target of 'continue' and 'break'
    //
    'outer_loop: loop {
        // loop label's name should start with a single quote
        println!("outer_loop");
        'inner_loop: while counter < 1000 {
            println!("inner_loop");
            if false {
                continue 'inner_loop; // we will never reach this one but we could use it too
            }
            // break            // by default break will apply on inner loop
            break 'outer_loop; // now we specify the loop label
        }
    }

    // 
    // looping a range of numbers (for i in range(10))
    // 
    for i in 0..3{          // start from 0
        for j in (0..2).rev(){  // start from 2
            println!("{i}:{j}")
        }
    }

    // 
    // looping an existing collection
    // 
    let numbers = [1, 2, 3,5];
    for i in numbers.iter().rev(){
        println!("{i}")
    }
}
