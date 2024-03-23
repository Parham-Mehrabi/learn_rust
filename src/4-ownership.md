# Ownership

__there are 3 essential rules in rust ownership:__  
1. ach value in Rust has an owner.  
2. There can only be one owner at a time.  
3. When the owner goes out of scope, the value will be dropped.  


## stack vs heap

if variable is using __stack__ like a regular int, it will have an actual value in memory for the variable  
otherwise if it is using a __heap__, it would be just a pointer which point to the variable which is called pointee  

in rust you cant have multiple pointer to a pointee (except you use borrowing), and when you use another variable to point previous one which uses heap, it will move the pointer to second one and first variable will be unusable,  
you can also move the pointer by sending it through a function
to avoid this you have 3 solutions:

1. if its a function return the argument with the return value too and use that

2. use .clone()

3. use borrowing (next lesson)


--- 

if you use .clone() it will create another pointer and pointee, in other words it will deep-copy it

---
some other data structures also use Box which is heap, like __Vec__, __String__ and __HashMap__
---