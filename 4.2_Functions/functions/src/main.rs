fn main() {
    print_number(32);

    print_sum(6, 7);

    int_plus_plus(9);

    // let 開頭都是陳述式
    // 「宣告陳述式」（declaration statements）
    let x: i32;
    // 「表達陳述式」（expression statements）
    //  : 把表達式轉為陳述式的方法
    // see: int_plus_plus()
    // 我們的函式聲明會回傳一個 i32，但是有分號時，它卻會回傳 ()。 Rust 覺得這應該不是我們想要的，所以在上面看到的錯誤訊息中建議我們移除分號。

    println!("{}", early_return(30));

    // 發散函式（Diverging functions），不回傳值
    // diverges(); // <-- This function call will crash application!

    // 函式指標（Function pointers）
    let f: fn(i32) -> i32; // f 是一個指向以 i32 作為參數且回傳 i32 的函式的變數綁定

    let f: fn(i32) -> i32 = int_plus_plus;
    // Or
    let f = int_plus_plus;

    let six = f(5);
    println!("{}", &six);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("x + y = {}", x + y);
}

fn int_plus_plus(x: i32) -> i32 {
    x + 1 // 此處沒有分號(;)
}

fn early_return(x: i32) -> i32 {
    return x;

    // x + 1 // <-- Never Run

    // 沒有 return 的定義可能讓你覺得看起來有些奇怪。 不過隨著時間過去，它會變得很直觀。
}

fn diverges() -> ! {
    // <-- Type of diverge functions is !
    panic!("Application Crash!!");
}
