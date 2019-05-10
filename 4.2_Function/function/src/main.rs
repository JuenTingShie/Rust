fn main() {
    print_number(5, 7);
    println!("30 + 1 = {}", add_one(30));
    println!("Early return value: {}", early_return(8));
    //Function Pointer//
    let a: fn(i32) -> i32 = add_one;
    let num = a(17);
    println!("a = {}", num);
}

fn print_number(x: i32, y: i32){
    println!("x: {} , y: {}", x, y);
}

fn add_one(x: i32) -> i32 {
    x + 1  //If not end by ->; it will send back this value by i32 type//
}

fn early_return(x: i32) -> i32 {
    return x;
    //This function will end here because the return//
    println!("x = {}", x);

    x + 1
}