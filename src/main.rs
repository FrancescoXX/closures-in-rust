/*Closure in Rust

Closures are block of code that can be :
- stored in variables
- passed as arguments to functions
- returned from functions

They are similar to functions but with one main difference: 

They can capture variables from their surrounding scope. 


Key features:
- **Anonymous Functions**: 
Closures are unnamed functions that can be stored in variables or passed to other functions.

- **Capturing Environment**: 
Closures can capture values from their surrounding scope by borrowing, mutably borrowing, or taking ownership of them.

- **Type Inference**: 
Rust infers the types of parameters and return types in most closures, so explicit type annotations are often unnecessary.

- **Flexibility**: 
Closures can be stored as function pointers or as traits like `Fn`, `FnMut`, and `FnOnce`, depending on how they capture variables.
*/

fn main() {
    // the syntax for closure is similar to function syntax
    // || { code }

    //example1
    let closure  = || "Hello, World!";

    println!("{}", closure());

    //example2
    let add = |a: i32, b: i32| a + b;

    println!("{}", add(2, 3));

    //example3
    let hello = String::from("Hello, ");
    let world = String::from("World!");

    let add2 = | a, b | a + b;

    println!("{}", add2(hello, &world));

    //Capturing Cariables with closures
    //There are 3 different ways to capture variables with closures:
    //1. Borrowing a variable immutably
    //2. Borrowing a variable mutably
    //3. Taking ownership of a variable

    //example4 - capturing by borrowing
    let x = 50;
    let print_x = || println!("{}", x); 
    print_x();

    //example5 - capturing by mutable borrowing
    let mut y = 100;

    let mut print_y = || {
        y += 1;
        println!("{}", y);
    };

    print_y();

    //example6 - capturing by taking ownership
    let z = String::from("Hello");

    let print_z = move || {
        println!("{}", z);
        drop(z);
    };

    print_z();

    // println!("z: {:?}", z); 

    //Closure traits
    // - Fn: captures variables by reference (&T)
    // - FnMut: captures variables by mutable reference (&mut T)
    // - FnOnce: captures variables by value (T)

    //example7 - closures as Function Parameters
    let double = |x| x * 2;
    apply(double);

}

//example7 - closures as Function Parameters
fn apply<F>(f: F) where F: Fn(i32) -> i32 {
    println!("{}", f(10)); // 20
}


/* 
Differences Between Functions and Closures

1. Capturing Variables: Closures can capture variables from the surrounding scope, whereas functions cannot.

2. Syntax: Closures are defined using the |args| body syntax, while functions use the fn keyword.

3. Flexibility: 
Closures can be stored in variables, passed around as arguments, and returned from other functions, giving them more flexibility than traditional functions.

4. Memory Usage: 
Closures that capture variables from their environment may use more memory than regular functions because they store those captured values.

*/