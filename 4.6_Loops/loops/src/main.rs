fn main() {
    // Endless Loop
    // loop {
    //     println!("hey !");
    // }

    // While Loop
    // let x = 5;
    // while x == 5 {
    //     println!("hehe");
    // }
    // For Loop
    for x in 0..10 {
        println!("{}", x);
    }

    for (i, j) in (5..10).enumerate() {
        println!("time: {}, number: {}", i, j);
    }

    let some_string = "hello\nworld".lines();
    for (count, word) in some_string.enumerate() {
        println!("{} word:{}", count, word);
    }

    let mut x = 5;
    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }

    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }

        println!("{}", x);
    }
    // 使用標籤去指定 break 或 continue 要作用在哪層迴圈。
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // continues the loop over x
            if y % 2 == 0 {
                continue 'inner;
            } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }
}
