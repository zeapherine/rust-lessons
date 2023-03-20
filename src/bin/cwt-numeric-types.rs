fn main(){
    let sum = 2 + 2;
    let value = 10 - 5;
    let div = 10/2;
    let mul = 5 * 5;


    let five = sub(8, 3);
    println!("five {:?}", five)
}

fn sub(a: i32, b: i32) -> i32{
    a-b
}