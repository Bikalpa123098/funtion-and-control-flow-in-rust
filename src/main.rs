fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5);
    another_function_with_multiple_parameters("Bikalpa", 20);
    statement_and_expression();
    println!("{}",return_function());
}

fn another_function(){
    println!("This is an another function!");
}

fn another_function_with_parameters(x:i32) {
    println!("The value of x is: {x} ..........");
}

fn another_function_with_multiple_parameters(name:&str, age:u8){
    println!("My name is {} and I am {} years old!",name,age);
}
fn statement_and_expression(){
    let x = {
        let y = 5;
        y+6
    };
    println!("{}",x);
}
fn return_function() -> i8{
    let x =5;
    x
}