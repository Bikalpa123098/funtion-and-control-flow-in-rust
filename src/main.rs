fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5);
    another_function_with_multiple_parameters("Bikalpa", 20);
}

fn another_function(){
    println!("This is an another function!");
}

fn another_function_with_parameters(x:i32) {
    println!("The value of x is: {}",x);
}

fn another_function_with_multiple_parameters(name:&str, age:u8){
    println!("My name is {} and I am {} years old!",name,age);
}