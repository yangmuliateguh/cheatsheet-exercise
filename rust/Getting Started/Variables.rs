// (Global) constant
const TEST: i64 = 9;

fn main (){
    // Initializing and declaring a variable
    let some_variable = "this is a variable";
    println!("{some_variable}");

    // Making a variable mutable
    let mut mutable_variable = 1;
    println!("{mutable_variable}");
    mutable_variable = 2;
    println!("{mutable_variable}");

    // Assigning multiple variables
    let (name, age) = ("string", 1);
    println!("{name} {age}");

    println!("{TEST}");

    const TEST2: i64 = 8;
    println!("{TEST2}");
}