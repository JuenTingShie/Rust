fn main() {
    let mut x = 5;
    let (a, b) = (1, 2);
    let y: i32 = 5;
    print!("x: {} \n", x);
    print!("a: {} \nb: {} \n", a,b);
    print!("y: {} \n", y);
    x = 10;
    print!("x: {} \n", x);
    ///shadowed///
    let c: i32 = 8;
    {///block///
        print!("c: {} Number from outer\n", c);
        let c = 12;
        print!("c: {} Number from block\n", c);
    }
    print!("c: {} Number from outer\n", c);
    let c = 40;
    print!("c: {} Number from another let\n", c);
}