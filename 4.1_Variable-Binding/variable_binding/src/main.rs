fn main() {
    // *Patterns
    let (x, y) = (10, 100);

    // *Type annotations
    let x: i32 = 5; // x 被綁定為 i32 型別，而且數值是 5

    // *Mutablility
    let x = 5; // 變數 x 預設為不可變

    // x = 10; // <-- 變數 x 不可變

    let mut x = 10; // 變數 x 預設為可變

    x = 100;

    // *Initializing bindings
    let i: i32;
    // println!("i is: {}", i); // <-- 未初始化的 i 不可被使用

    // *Scope and shadowing
    let a: i32 = 7;
    {
        let b: i32 = 63;
        println!("a: {}, b: {} ", a, b);
    }
    // println!("a: {}, b: {} ", a, b); // <-- 變數 b 不在可用範圍內

    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8"
        let x = 12;
        println!("{}", x); // Prints "12"
    }
    println!("{}", x); // Prints "8"
    let x = 42;
    println!("{}", x); // Prints "42"
}
