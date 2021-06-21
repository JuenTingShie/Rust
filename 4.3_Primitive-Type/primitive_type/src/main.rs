fn main() {
    // 布林（Booleans）
    let x = true;
    let y: bool = false;

    // char 代表一個 Unicode 值
    // Rust 的 char 並非一個位元組，而是四個位元組
    let x = 'x';
    let two_hearts = '💕';

    println!("{}", two_hearts);

    // 數字
    let x = 42; // x has type i32

    let y = 1.0; // y has type f64

    // 陣列（Arrays）
    // 陣列的Type是 [T;N], T for Type, N for count of Number
    let a = [10, 20, 30, 40]; // a: [i32; 4]
    println!("len of a is: {}", a.len());

    let b = [0; 20]; // b: [i32; 20]
    println!("{}", b.len());

    let names = ["Sam", "John", "Joey"];
    println!("{}", names[1]);

    // 分割（Slices）
    // 分割的Type是 &[T]
    let a = [1, 2, 3, 4, 5, 6];
    let all = &a[..];
    println!("{:?}", all);

    let cuts = &a[1..3];
    println!("{:?}", cuts);

    // 多元組（Tuples）
    let x = (1, "hello", 4.2);

    let (a, b, c) = x;

    println!("{}", b);

    // 多元組索引（Tuple Indexing）
    // 就像陣列的索引，它從零開始，但是不像陣列所以它使用 . 而不是 []。
    println!("{}, {}, {}", x.0, x.1, x.2);
}
