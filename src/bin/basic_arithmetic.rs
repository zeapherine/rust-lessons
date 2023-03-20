fn main(){
    let result = add(6, 18);
    print(result);
}

fn add(a: i32, b: i32)-> i32 {
    a+b
}

fn print(num: i32){
    println!("The result is {:?} ", num);
}