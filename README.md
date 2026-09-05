# MSH

**MSH** stands for **Modern Shell**.

I wanted to make a shell language that has a syntax like general purpose programming languages such as Python, C, Rust, and Java.

Currently, it's under development and in a very early stage.

## Intended Syntax for The Future

```msh
function main() -> Void
    loop 
        let num: Number = if not input!("Enter number 1: ").parse()
            print!("{}", Error("you must enter a number"))
            continue 
        let num: Number = if not input!("Enter number 1: ").parse()
            print!("{}", Error("you must enter a number"))
            continue 
        let op: String = input!("Enter operator: ")
        let result: Number = match op 
            "+" => num1 + num2, 
            "-" => num1 - num2, 
            "*" => num1 * num2,
            "/" => \ 
                if num2 == 0 
                    print!("{}", Error("You cannot divide by zero"))
                    continue 
                num1 / num2 
            _ => \
                print!("{}", Error("invalid operator"))
                continue 
        print!("{}", result)

main!()
```

```msh 
function first_even(numbers: [Number]) -> Option<Number>
    for num in numbers
        if num % 2 == 0
            return Some(num)

    return None


function main() -> Void
    let numbers: [Number] = [1, 3, 7, 10, 13, 18]

    let result: Option<Number> = first_even!(numbers)

    match result
        Some(num)
            print!("First even number: {}", num)

        None
            print!("{}", Error("there are no even numbers"))


main!()
```

```msh 
function main() -> Void
    let directory: String = input!("Directory: ")

    cd(directory)
        .if_fails!(|err|
            print!("{}", err)
            return
        )

    print!("Files in {}:", directory)

    ls()

    print!("\nSearching for Rust files...")

    find(".", "-name", "*.rs")
        .if_fails!(|err|
            print!("{}", err)
            return
        )

    print!("\nCounting Rust files...")

    find(".", "-name", "*.rs")
        | wc("-l")

    print!("\nLargest files:")

    du("-ah", ".")
        | sort("-h")
        | tail("-n", "10")


main!()
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

