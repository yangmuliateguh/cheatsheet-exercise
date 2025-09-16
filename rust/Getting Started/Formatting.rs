fn main (){
    // Single Placeholder
    println!("{}", 1);

    // Multiple Placeholder
    println!("{} {}", 1, 3);

    // Positional Arguments
    println!("{0} is {1} {2}, also {0} is a {3} programming language",
        "Rust", "cool", "language", "safe"
    );

    // Named Arguments
    println!("{country} is a nation", country = "indonesia");

    // Placeholder traits :b for binary, :0x is for hex and :o is octal
    println!("binary of 76 : {:b}\nhex of 76 : {:0x}\noctal of 76 : {:o}",76,76,76);

    // Debug Trait
    println!("Debug trait {:?}", (76, 'A', 90));

    // New Format Strings in 1.58
    let x = "world";
    println!("hello {x}");
}