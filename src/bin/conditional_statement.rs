fn main(){
    check_age(18);
    check_age(21);
    check_age(25);

    
}

fn check_age(age: i32){
    if age >= 21 {
        println!("Ok to purchase");
    } else {
        println!("Cannot purchase");
    }
}