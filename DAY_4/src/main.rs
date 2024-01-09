pub mod my_module;

fn main() {

    // Calling a function
    funPrintHelloWorld();

    // Calling a function with parameter
    funPrintHelloWorldWithParam("Monkey");

    // Calling a function with parameter and return value
    funPrintHelloWorldWithParamAndReturn("Monkey");
   
   // Calling a module
    my_module::hello::say_hello();

    // Calling a nested module
    my_module::myfuns::mymods::printhellonested();


}

// Creating a hello world function

fn funPrintHelloWorld() {
    println!("Hello, world!");
}


// Creating a hello world function with parameter
fn funPrintHelloWorldWithParam(name: &str) {
    println!("Hello, {}!", name);
}

// Creating a hello world function with parameter and return value

fn funPrintHelloWorldWithParamAndReturn(name: &str) -> String {
    return format!("Hello, {}!", name);
    
}










