// comment lol

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5);
    another_function_with_multiple_parameters("Bikalpa", 20);
    statement_and_expression();
    println!("{}",return_function());
    if_statement(55);
    notzero();
    divide_by_four();
    using_if_in_late_statement();
}


// functions
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


//control flow

fn if_statement(x:i8){
    if x<18{
        println!("Minor");
    }
    else{
        println!("Not Minor")
    }
}
fn notzero(){
    let n= 10;
    if n!=0{
        println!("{n} is not a zero");
    }
}
fn divide_by_four() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn using_if_in_late_statement(){
    let age = 17;
    let person = if age<18 {"Minor"} else {"Not a Minor"};
    println!("This person is {person}");
}