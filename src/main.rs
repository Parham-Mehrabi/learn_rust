fn main() {


    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();                               // you cant mutate `s` when there is a reference to it
    println!("the first word is: {}", word);
    s.clear();                                  // now the reference lifetime is over



    // 
    // slice references are fat references because they keep the metadata
    // 
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );


    // 
    // slices are not only for strings
    // 
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
