# functions

1. if we dont specify what to return:

    - it will return the last line which has no semicolon.  
    <small>if we have a return in next lines, we will get an error for not putting the semicolon in the lane</small>
    ```
    fn test() -> i32 {

        5
    }
    //  this function will return number 5.

    ```
    - if no such line, it will return ()
    ```
    fn test(){
        5;
    }

    // this function will NOT return number 5 but () instead
    ```

2. default return type is '()', called unit, is a empty tuple, so if we are returning that, we dont need to specify the type of returned value
```
fn my_func5(){
    return ()
}
```

3. unlike python, in rust, if we are going to return something else, we __have to__ specify the returned type !!!
```
fn return_five() -> i32{
    return 5;
}
```