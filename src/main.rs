fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5);
}

fn another_function(){
    println!("This is an another function!");
}

fn another_function_with_parameters(x:i32) {
    println!("The value of x is: {}",x);
}