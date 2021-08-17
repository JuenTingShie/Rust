/*
借出一個指向他人擁有的資源的 reference 是很複雜的。 舉例來說，想像一下以下的操作行為：

    我獲得一個某種資源的控制代碼（handle）。
    我把指向這個資源的 reference 借給你。
    我決定不再使用這個資源，釋放它，但你仍還有你那邊的 reference。
    你決定要使用此資源。

噢！你的 reference 指向了一個無效的資源。 如果資源是記憶體時，我們叫它迷途指標（dangling pointer）或「釋放後使用」。

要修正這個問題，我們必須確保上述第四步不會在第三步之後發生。
Rust 的所有權系統透過一個叫做生命週期（lifetimes）的概念達成這點，它會描述 reference 的有效範圍。

當我們有一個函式透過參數取用了一個 reference，我們可以不言明（implicit）或言明（explicit）reference 的生命週期：
*/

fn main() {
    struct Foo<'a> {
        x: &'a i32,
    }
    impl<'a> Foo<'a> {
        fn x(&self) -> &'a i32 {
            self.x
        }
    }

    let f = Foo { x: &4 };

    println!("{}", f.x);

    let x = 4;
    {
        let y = &5;
        let f = Foo { x: y };
        //x = &f.x; <- error here
    }
    println!("{}", x);

    /*
    名稱叫做 static 的生命週期是特殊的生命週期。
    */
    let words: &'static str = "Hello World";
    println!("{}", words);
}

// implicit
fn foo(x: &i32) {}

/*
explicit
'a 唸作「生命週期 a」
*/
fn bar<'a>(x: &'a mut i32) {}
