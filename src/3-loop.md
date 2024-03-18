# loops:

## 1. __loop__:
- this is simply a __'while True'__ loop but with some extra features
- you can __return a value__ in their break and then put it into a variable
    ```
    let result = loop {
        break "value"
    }
    ```

## 2. __for__:
- just like python's for loop but instead of range, here we use (start..end)
    ```
    for number in (1..4).rev() {
        println!("{number}!");
    }
    // it is better to start from the end so we use .rev() to reverse the iterable
    ```
- looping a collection is also available here  

## 3. __while__:
just like python3's while loop without a 'else' statement

## labels:
- by default 'break' and 'continue' statements are effecting the deepest loop, if we are going to break or continue some upper loops we have to use labels

- loop labels are declaring using a single quote and can be used for any kind of loop
    ```
    'outer: loop{
        'inner_loop: loop {
            break 'outer;
        }
    }
    ```