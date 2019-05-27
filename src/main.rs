fn main() {
    println!("Hello, world!");
    say_name_and_age("Mathieu", 32);
    // invalid_fn();
    add_five(5);

    let sixty_four = return_64();
    println!("Sixty four as a number is {}", sixty_four);

    let plus_one = return_plus_1(sixty_four);
    println!("{}", plus_one)
}

// Ugh snake case convention
// Fn arguments have to be explicitly typed
fn say_name_and_age(name: &str, age: i32) {
    println!("My name is {}", name);
    println!("I am {}", age);
}

// Can't assign a statement, let expect an expression (with a return value)
// Assignments do not return any values, they are statements, not expressions
// fn invalid_fn() {
//     let name = (let y = "Mathieu");
// }

fn add_five(x: i32) {
    let five = 5;

    // The scope created returns a value to be assigned to with_five_added
    // It is an expression, therefore valid for assignment
    let with_five_added = {
        let arg = x;
        // No semicolon for expressions
        // Adding a semicolon to an expression turns it into a statement
        arg + five
    };

    println!("{} plus five is {}", x, with_five_added);
}

// Typing return value of a function with ->
fn return_64() -> i32 {
    // Fns implicitly return the last expression
    64
    // Also possible
    // return 64
}

fn return_plus_1(x: i32) -> i32 {
    x + 1
    // Adding a semicolon doesn't return anything because it becomes a statement
    // Return value becomes (), empty tuple
    // x + 1;
}
