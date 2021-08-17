fn main() {
    // 借用（Borrowing）
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (mut v1, mut v2, answer) = foo(v1, v2);
    // 以上寫法不符合 Rust 習慣跟利用 借用 的優點

    // 第一步優化: (使用參照 reference)
    let ans = new_foo(&v1, &v2);

    v1[1] = 0;
    v2[2] = 0;

    println!("v1[1]: {}, v2[2]: {}", v1[1], v2[2]);

    // 補充: &mut reference
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1; // y : &mut i32，* 對其取值
    }
    println!("x: {}, ans: {}", x, ans);

    // 借用（Borrowing）的規則
    /*
    首先，任何借用的有效範圍都必須比擁有者的有效範圍還要小。 其次，你可以使用以下兩種借用，但是不能同時使用兩者：
        一到多個對資源的 reference（&T）
        唯一一個可變 reference（&mut T）
    */

    // 對有效範圍的深思（Thinking in scopes）
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s; // <-- Can't reference to the same &mut more then once!

        // println!("{}, {}", r1, r2);
    }
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // do stuff with v1 and v2

    // hand back ownership, and the result of our function
    (v1, v2, 42)
}

fn new_foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    /*
    我們把 &T 型別稱為「參照」（reference），它借用了所有權，而非掌握所有權。
    一個借用東西的綁定不會在離開有效範圍時把資源釋放掉。
    這代表在呼叫 foo() 完之後，我們仍可再度使用我們原始的綁定。

    reference 是不可變的（immutable），所以 v1 與 v2 都不能被修改
    */

    // v1[1] = 0; <-- reference cannot be change

    // return i32 value
    v1[1] + v2[2]
}
