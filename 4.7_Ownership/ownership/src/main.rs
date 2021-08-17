use std::vec;

fn main() {
    foo();
    /* 移動語意（Move semantics）
    Rust 確保所有的資源都 只有一個 對應的綁定。
    */
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("v[0] is: {}", v[0]); // <-- here will error because value of v is move to v2

    let v = vec![1, 2, 3];
    take(v);
    // println!("v[0] is: {}", v[0]); // <-- here will error because value of v is move to take()'s v

    // 細節
    let x = 10;
    // Rust 會替 i32 配置記憶體在 堆疊 上，把代表 10 的位元複製到配置好的記憶體中，並綁定變數名稱 x 到此區記憶體以便未來使用。

    /*
    Rust 會複製堆積所配置的記憶體位址給內部指標，這個內部指標是向量物件實際存在堆疊上的部份（讓我們稱它資料指標 data pointer）。
    最重要的是，向量物件和它的資料分別放在不同的記憶體區塊，而不在單一連續的記憶體配置中（因為一些理由我們不會細說）。
    向量的這兩部分（一部份在堆疊、一部份在堆積）必須在任何時候都與對方一致，像長度、容量等。
    */

    let v = vec![1, 2, 3];
    let mut v2 = v;
    /* 我們移動 v 到 v2 時，Rust 實際上是把向量物件 v 中的位元一個一個地複製到堆疊中配置的 v2。
    這份複本並沒有建立堆積配置中的實際資料的複本。 這也代表，會同時有指到向量實際內容的兩個指標，兩者都指向堆積中的同一塊記憶體配置。

    當 let mut v2 = v 時，只是把 v 中的資料指標複製一份給 v2，所以 v 與 v2 同時指向堆積中的同一份資料。 當兩者同時存取時就會出現資料競爭的可能性。
    */

    // Copy 型別
    /*
    所有的基本型別都實作了 Copy trait，也因此它們的所有權並不會像「所有權規則」所假設的那樣被移動。
    */
    let v = 1;
    let v2 = v;
    println!("v is: {}", v);

    // 所有權以外（More than ownership）

    /*
    如果我們必須在每個函式都交還所有權：
    這將會非常煩人。 當我們想處理更多所有權的時候會變得更糟：

    呃！ 回傳型別、回傳的那行程式碼、和呼叫函式都變得更複雜了。

    幸運的是，Rust 提供一個功能，借用（borrowing），可以幫助我們解決這個問題。 這就是下一節的主題！
        */
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, answer) = bar(v1, v2);
}

fn foo() {
    /*
    當 v 進入有效範圍時，一個新的 向量（vector）會在 堆疊（stack）中被建立，且在 堆積（heap）中替其元素配置空間。
    當 v 在 foo() 的最後離開有效範圍時，Rust 會清除任何與這個向量有關的東西，甚至是堆積內配置的記憶體。 這在有效範圍結束後必定發生。
    */
    let v = vec![1, 2, 3];
}

fn take(v: Vec<i32>) {
    // what happens here isn’t important.
}

fn bar(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // do stuff with v1 and v2

    // hand back ownership, and the result of our function
    (v1, v2, 42) // return ownership back is so annoying
}
