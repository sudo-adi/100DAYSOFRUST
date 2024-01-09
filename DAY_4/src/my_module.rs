
pub mod hello {
    // Code within the module
    pub fn say_hello() {
        println!("Hello from my_module!");
    }

    pub mod nested_module {
        pub fn greet() {
            println!("Greetings from nested_module!");
        }
    }
}



pub mod myfuns{
    pub mod mymods{
        pub fn printhellonested(){
            println!("hello nested module");
        }
    }
}