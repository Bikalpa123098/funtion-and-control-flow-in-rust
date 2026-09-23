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
    // loops();
    return_value_from_the_loop();
    while_loop();
    while_loop_in_index();
    for_loop_in_index();
    range();
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

//loopssssssss

fn loops(){
    loop{
        let x =0;
        println!("{x}");       
        // x=x+1;

    }
}

fn return_value_from_the_loop(){
    let mut counter = 0;
    let result = loop{
        counter+=1;
        if counter==10{
            break counter*2;
        }
    };
    println!("{}",result);
}
fn while_loop(){
    let mut num = 3;
    while num!=0 {
        println!("{num}!");
        num-=1;
    }
    println!("aabra ka badra");
}
fn while_loop_in_index(){
    let a = [1,2,3,4,5];
    let mut i = 0;
    while i <5 {
        println!("{}",a[i]);
        i+=1;
    }
}
fn for_loop_in_index(){
    let a = [10,20,30,40,50];
    for element in a  {
        println!("{}",element);
    }
}

fn range(){
    for n in (1..10){
        println!("{n} is from range");
    }
    for m in (0..5).rev(){
        println!("{} in rev",m);
    }
}