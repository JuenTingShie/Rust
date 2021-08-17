fn main() {
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    }

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // if 是個表達式。 表達式的值就是任何一個被選擇的分支的最後一個表達式的值。 一個沒有 else 的 if 總是會回傳 () 值。

    // 類似於 c++ 或 csharp 的三元運算子
    // y = (x==true) ? 10 : 20;
    let y = if x == 5 { 10 } else { 15 }; // y: i32
    println!("{}", y);
}
