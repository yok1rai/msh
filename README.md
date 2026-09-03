# MSH

**MSH** stands for **Modern Shell**.

I wanted to make a shell language that has a syntax like general purpose programming languages such as Python, C, Rust, and Java.

Currently, it's under development and in a very early stage.

## Intended Syntax for The Future

```msh
const MIN_AGE: Number = 18 

function first_even(arr: [Number]): May[Number]
    for num of arr 
        if num % 2 == 0 
            return Found(num) 
    return None 

numbers: [Number] 

print!("Enter 5 numbers ")
for i of 1..=5
    num: Number = read("-p", format!("Number {}: ", i))
    numbers.append(num) 
if num found first_even!(numbers)
    print("First even number is {}", num)
else 
    print!("There is no even number in this array")
```

## Current Development

MSH is currently in a very early stage of development.

The current implementation is focused on building the underlying shell functionality in Rust, including:

* Builtin commands
* External command execution
* Process creation with `fork()`
* Program execution with `execvp()`
* Process synchronization with `waitpid()`
* Signal handling
* Command input and parsing

The long-term goal is to evolve these foundations into the MSH language described above.

## License

This project is licensed under the [MIT License](./license).

