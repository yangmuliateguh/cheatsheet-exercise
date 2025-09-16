fn main (){
    let mut a: u32 = 0;
    println!("u32 min: {}", u32::MIN);
    println!("u32 max: {}", u32::MAX);
    println!("{a}");
    a = 8;
    println!("{a}");

    let b: u64 = 877;
    println!("u64 min: {}", u64::MIN);
    println!("u64 max: {}", u64::MAX);
    println!("{b}");

    let c: i64 = -8999;
    println!("i64 min: {}", i64::MIN);
    println!("i64 max: {}", i64::MAX);
    println!("{c}");

    let d = -90;
    println!("{d}");
}